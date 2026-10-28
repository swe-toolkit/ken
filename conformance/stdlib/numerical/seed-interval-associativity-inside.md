# Interval arithmetic associativity by endpoint-value inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §6.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE.md`
(as amended by the Steward, `evt_4b8jdb01qbht0`).

Every case promises a durable invariant. `MkRational n k` is
`n/(Suc k)`. The three non-identical intervals are
`i=[1/2,1]`, `j=[1/3,2]`, and `k=[3/2,3]`, with six distinct
endpoint values. The accepted client applies all four public laws,
one per operation and grouping direction; two direct `Proved`
inclusions check the computed values without invoking a new law.
Closed regroupings of these inputs have **identical stored
spellings**: both structural `Equal Interval` propositions accept
by `Refl`. This is not evidence for a generic structural theorem,
and a closed instance does not prove the four generic inclusion
laws. Open regroupings are not definitionally convertible; those
are the ones the generic laws address.

## Accepted client

Check the **entire** client before appending the sole refusal. The
last two structural controls must not be interpreted as generic
`Equal Interval` associativity.

```ken
import Data.Numeric.Rational (Rational, MkRational)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_add,
    interval_mul,
    interval_inside,
    interval_add_assoc_inside,
    interval_add_assoc_rev_inside,
    interval_mul_assoc_inside,
    interval_mul_assoc_rev_inside)

const third : Rational = MkRational (Suc Zero) (Suc (Suc Zero))

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const one : Rational = MkRational (Suc Zero) Zero

const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)

const two : Rational = MkRational (Suc (Suc Zero)) Zero

const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const i : Interval = MkInterval half one

const j : Interval = MkInterval third two

const k : Interval = MkInterval three_halves three

theorem add_regroup
    : IsTrue
        (interval_inside
          (interval_add i (interval_add j k))
          (interval_add (interval_add i j) k)) =
  interval_add_assoc_inside i j k

theorem add_regroup_back
    : IsTrue
        (interval_inside
          (interval_add (interval_add i j) k)
          (interval_add i (interval_add j k))) =
  interval_add_assoc_rev_inside i j k

theorem mul_regroup
    : IsTrue
        (interval_inside
          (interval_mul i (interval_mul j k))
          (interval_mul (interval_mul i j) k)) =
  interval_mul_assoc_inside i j k

theorem mul_regroup_back
    : IsTrue
        (interval_inside
          (interval_mul (interval_mul i j) k)
          (interval_mul i (interval_mul j k))) =
  interval_mul_assoc_rev_inside i j k

theorem add_regroup_direct
    : IsTrue
        (interval_inside
          (interval_add i (interval_add j k))
          (interval_add (interval_add i j) k)) =
  Proved

theorem mul_regroup_direct
    : IsTrue
        (interval_inside
          (interval_mul (interval_mul i j) k)
          (interval_mul i (interval_mul j k))) =
  Proved

theorem prefix_inside_sum : IsTrue (interval_inside (interval_add i j) (interval_add i j)) =
  Proved

theorem sum_not_inside_prefix
    : Equal Bool
        (interval_inside (interval_add i (interval_add j k)) (interval_add i j))
        False =
  Proved

theorem add_regroupings_coincide
    : Equal Interval (interval_add i (interval_add j k)) (interval_add (interval_add i j) k) =
  Refl

theorem mul_regroupings_coincide
    : Equal Interval (interval_mul i (interval_mul j k)) (interval_mul (interval_mul i j) k) =
  Refl
```

### stdlib/numerical/interval-add-assoc-both-directions

- spec: §6, `interval_add_assoc_inside` and
  `interval_add_assoc_rev_inside`.
- promise class: durable invariant.
- given: three distinct-endpoint rational intervals, each with a
  different low and high endpoint.
- expect: `add_regroup` and `add_regroup_back` accept, and the
  independent direct inclusion and closed `Refl` equality accept.
- measured: two public-law applications and one direct `Proved`
  inclusion at this closed triple; `Refl` establishes structural
  identity **only here**.
- claimed: the two regroupings mutually include by endpoint values
  for **all** interval triples.
- gap: these closed checks do not prove either generic law; the
  generic proof is a separate checked catalog obligation.
- why: opposite applications pin availability and orientations of
  both public laws; a single law cannot supply both by permutation.

### stdlib/numerical/interval-mul-assoc-both-directions

- spec: §6, `interval_mul_assoc_inside` and
  `interval_mul_assoc_rev_inside`.
- promise class: durable invariant.
- given: the same three intervals, with mixed denominators.
- expect: `mul_regroup` and `mul_regroup_back` accept; the direct
  inclusion and closed `Refl` equality accept independently.
- measured: both law applications and one direct `Proved` inclusion
  plus structural identity at this closed triple.
- claimed: multiplication regroupings mutually include by
  endpoint values, even for inverted intervals.
- gap: this ordered, closed triple does not exercise inverted
  arguments or discharge the generic quantifiers; the exported
  proof bodies must.
- why: direct controls reduce closed arithmetic rather than simply
  reusing the two public laws as their own oracles.

### stdlib/numerical/prefix-sum-same-shape-control

- spec: §3.1, `interval_inside`, and §6's limit.
- promise class: durable invariant.
- given: `interval_add i j`, the **same containing second operand**
  used by the isolated false claim below.
- expect: `prefix_inside_sum : IsTrue (interval_inside _
  (interval_add i j)) = Proved` accepts; independent
  `sum_not_inside_prefix` checks that the false claim's Boolean
  value is `False`.
- measured: a same-form accepted proposition against the same
  container, and a direct `Equal Bool ... False` on the false input.
- claimed: the refused block fails because the three-operand sum
  exceeds this two-operand prefix sum, not because the form or fixture
  is invalid.
- gap: this one fixture does not characterize arbitrary inclusion.
- why: adding `k` grows the upper endpoint from `3` to `6` while
  the containing prefix sum's upper endpoint stays `3`.

## Isolated refused inclusion

On a **fresh complete copy** of the accepted client append **only**
`seed_neg` below. The accepted `prefix_inside_sum` has the same
`IsTrue (interval_inside _ (interval_add i j)) = Proved` form;
`sum_not_inside_prefix` independently confirms that the proposed
inclusion is `False`. The refusal must land inside the appended
theorem, never at an import, a law application or an earlier fixture.

### stdlib/numerical/reject-three-operand-sum-inside-prefix

- spec: §6 and §3.1.
- promise class: durable invariant.
- given: the same `i`, `j`, `k` and their three-operand sum.
- expect: reject the claim that the regrouped sum lies inside the
  sum of `i` and `j` alone.
- measured: on the same accepted fixture the value is `False`;
  appending the false `IsTrue` proof was refused in Architect D0.
- claimed: associativity never authorizes dropping the third
  operand from one side of the inclusion.
- gap: one refusal does not prove a universal limit, but it rules
  out this concrete false weakening.
- why: the high endpoint `6` is not `≤ 3`.

```ken
theorem seed_neg
    : IsTrue (interval_inside (interval_add i (interval_add j k)) (interval_add i j)) =
  Proved
```

## Evidence boundary

Architect's D0 on detached `edc1da359` with local patch SHA-256
`81f9dfa9edb42ca7f37263dec182dff8ff7530e0e8167d4f8744e1f1f1013556`
checked four generic proof bodies, with `ken check` and
`ken fmt --check` rc 0, no new `Axiom`, exact scratch trusted
GlobalIds 114 → 114, and three probes passed. Its positive client
accepted whole; appending only the proposed `seed_neg` was refused
with `KernelRejected TypeMismatch`, after the accepted same-form and
`False` controls. These are scratch observations of the proposed
catalog proof, not shipped §6 exports or a generic proof from
closed instances.

The author separately extracted the exact accepted and refusal
Ken blocks (byte-identical to Architect's sources) to a detached
`edc1da359` scratch worktree. With only the D0 patch applied and
a preexisting shared Ken CLI binary, the whole accepted client
checked rc 0; a fresh whole accepted prefix plus only `seed_neg`
failed rc 1/K0004 `KernelRejected TypeMismatch` at bytes 2464–2577
inside the appended theorem. On the unpatched base, the independent
value and `Refl` controls without absent law imports or applications
checked rc 0; the full client rejected at the absent public
`interval_add_assoc_inside` import, rc 1/E0012 `UnboundName`.
Logs under `/workspaces/ken/local/`:
`spec-author-assoc-positive.log`, `spec-author-assoc-false.log`,
`spec-author-assoc-base-values.log` and
`spec-author-assoc-base-laws.log`.
The patch was reversed and the scratch worktree removed. The shared
binary was not independently built at the exact detached base, and
none of these earlier closed checks proves the generic law. The catalog
candidate now exports all four checked laws; its targeted package test
root-loads the whole accepted seed, rejects the isolated false claim,
and measures unchanged trusted GlobalIds. This is candidate package-test
execution, not official conformance, CI, publication, landing or landed
trust.
