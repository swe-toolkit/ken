# Hull and intersection regroup by endpoint-value inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §9.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ASSOCIATIVITY.md`.

Every case promises a durable invariant. Reuse the same three intervals as
`seed-interval-associativity-inside.md`: `i=[1/2,1]`, `j=[1/3,2]`, and
`k=[3/2,3]`. The accepted client applies each of §9's four public laws to
this closed triple. Those instantiations do not prove the laws' generic
quantifiers or assert structural associativity.

The refused hull inclusion uses the exact same expression as an accepted
`False` value control. By §2.1's endpoint formulas, the nested hull has
endpoints `[1/3,3]`, while the containing prefix hull has `[1/3,2]`. By
§3.1, the latter does not contain the former because `3 ≤ 2` is false.

## Accepted client

The whole accepted client must check before the refusal is considered.
Append only `seed_neg` from the isolated block below to a fresh complete copy.

```ken
import Data.Numeric.Rational (Rational, MkRational)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_hull,
    interval_intersect,
    interval_inside,
    interval_hull_assoc_inside,
    interval_hull_assoc_rev_inside,
    interval_intersect_assoc_inside,
    interval_intersect_assoc_rev_inside)

const third : Rational = MkRational (Suc Zero) (Suc (Suc Zero))

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const one : Rational = MkRational (Suc Zero) Zero

const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)

const two : Rational = MkRational (Suc (Suc Zero)) Zero

const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const i : Interval = MkInterval half one

const j : Interval = MkInterval third two

const k : Interval = MkInterval three_halves three

theorem hull_right_inside_left
    : IsTrue
        (interval_inside
          (interval_hull i (interval_hull j k))
          (interval_hull (interval_hull i j) k)) =
  interval_hull_assoc_inside i j k

theorem hull_left_inside_right
    : IsTrue
        (interval_inside
          (interval_hull (interval_hull i j) k)
          (interval_hull i (interval_hull j k))) =
  interval_hull_assoc_rev_inside i j k

theorem intersect_right_inside_left
    : IsTrue
        (interval_inside
          (interval_intersect i (interval_intersect j k))
          (interval_intersect (interval_intersect i j) k)) =
  interval_intersect_assoc_inside i j k

theorem intersect_left_inside_right
    : IsTrue
        (interval_inside
          (interval_intersect (interval_intersect i j) k)
          (interval_intersect i (interval_intersect j k))) =
  interval_intersect_assoc_rev_inside i j k

theorem dropped_operand_value_is_false
    : Equal Bool
        (interval_inside
          (interval_hull i (interval_hull j k))
          (interval_hull i j))
        False =
  Proved
```

### stdlib/numerical/hull-assoc-both-directions

- spec: §9, `interval_hull_assoc_inside` and
  `interval_hull_assoc_rev_inside`.
- promise class: durable invariant.
- given: the fixed `i`, `j`, and `k` triple above.
- expect: both public law applications and the direct `False` value
  control accept.
- measured: two generic theorem applications and the closed Boolean
  value of the dropped-operand inclusion.
- claimed: either hull grouping is inside the other for all triples,
  including inverted intervals.
- gap: these closed applications do not establish either generic law.
- why: the two law applications preserve the distinct directions; the
  closed controls ground the separate dropped-operand refusal.

### stdlib/numerical/intersect-assoc-both-directions

- spec: §9, `interval_intersect_assoc_inside` and
  `interval_intersect_assoc_rev_inside`.
- promise class: durable invariant.
- given: the same fixed `i`, `j`, and `k` triple.
- expect: both public law applications accept.
- measured: the two law applications at this closed triple.
- claimed: either intersection grouping is inside the other for all
  triples, including inverted intervals.
- gap: this ordered input does not independently establish the generic
  laws or test every inverted-input position.
- why: opposite theorem applications pin availability and direction for
  the two intersection laws.

### stdlib/numerical/dropped-hull-operand-value-control

- spec: §9 and §3.1, `interval_inside`.
- promise class: durable invariant.
- given: the same three intervals and the same hull-inclusion expression
  used by the isolated refusal below.
- expect: `dropped_operand_value_is_false` accepts for the exact
  dropped-operand expression.
- measured: the closed Boolean value for that expression.
- claimed: the false inclusion is caused by the dropped upper endpoint,
  rather than malformed setup or an unavailable public form.
- gap: this is one closed counterexample, not a universal characterization
  of interval inclusion.
- why: §3.1 checks endpoint values, so the inner upper endpoint `3` fails
  the outer upper bound `2`.

## Isolated refused inclusion

On a fresh complete copy of the accepted client, append only `seed_neg`.
The accepted direct `False` control uses the identical inclusion expression;
no diagnostic spelling is pinned.

### stdlib/numerical/reject-hull-with-dropped-operand

- spec: §9 and §3.1.
- promise class: durable invariant.
- given: the accepted `i`, `j`, and `k` and the three-operand nested hull.
- expect: reject the false inclusion claim at `seed_neg`.
- measured: the accepted client checks the identical expression as
  `False`; Architect's prior scratch D0 refused this `IsTrue` claim.
- claimed: the §9 regrouping laws do not authorize dropping `k` from the
  containing hull.
- gap: this one refusal rules out this concrete false weakening only.
- why: the inner upper endpoint is `3`, while the prefix hull's upper
  endpoint is `2`.

```ken
theorem seed_neg
    : IsTrue
        (interval_inside
          (interval_hull i (interval_hull j k))
          (interval_hull i j)) =
  Proved
```

## Evidence boundary

Architect's AC-0 was detached scratch evidence on base
`903b06068d7c67c74ab9d979513c31c0c7572b31`. It checked the four proposed
laws and closed controls, accepted the positive client, and refused the
same false `IsTrue` claim. It predates this seed and is not execution of
this file. The catalog package test and candidate-local mutation evidence
are also separate from execution of this conformance seed.

This CV turn performed static structure and source checks only. No Ken
check or official conformance-runner execution of this seed, CI,
publication, landing, or landed-trust result is claimed. No external or
`local/refs` material was consulted.
