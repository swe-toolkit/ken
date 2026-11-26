# Rational interval split hull

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §23.

These cases exercise both public split-hull laws. The sound direction
requires `lo ≤ m ≤ hi`; the complete direction has no order premises
and also covers exterior split points. Open clients pin the public
directions and premise counts. Closed interior and exterior cases
separate the sound conclusion by value.

Closed claims and order-premise arguments use `Proved`; no `Refl` is
used for an `IsTrue` goal. The shared accepted prefix is one source unit.

## Shared accepted prefix

```ken
import Core.Classes.LawfulClasses (IsTrue)

import Data.Numeric.Rational
  (Rational, MkRational, rational_leq)

import Algorithm.Numeric.RationalInterval
  (MkInterval, interval_contains, interval_hull, interval_inside,
   interval_inside_hull_left, interval_inside_hull_right,
   interval_inside_sound, interval_inside_split_hull,
   interval_inside_trans, interval_split_hull_inside)

const zero : Rational = MkRational Zero Zero

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const one : Rational = MkRational (Suc Zero) Zero

const two : Rational = MkRational (Suc (Suc Zero)) Zero

theorem split_hull_inside_open
      (lo : Rational) (hi : Rational) (m : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → IsTrue
        (interval_inside
          (interval_hull (MkInterval lo m) (MkInterval m hi))
          (MkInterval lo hi)) =
  λlower.
    λupper.
      interval_split_hull_inside lo hi m lower upper

theorem inside_split_hull_open
      (lo : Rational) (hi : Rational) (m : Rational)
    : IsTrue
        (interval_inside
          (MkInterval lo hi)
          (interval_hull (MkInterval lo m) (MkInterval m hi))) =
  interval_inside_split_hull lo hi m

theorem split_lower_member_in_whole
      (lo : Rational) (hi : Rational) (m : Rational) (x : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → IsTrue (interval_contains (MkInterval lo m) x)
      → IsTrue (interval_contains (MkInterval lo hi) x) =
  λlower.
    λupper.
      λmember.
        interval_inside_sound
          (MkInterval lo m)
          (MkInterval lo hi)
          x
          (interval_inside_trans
            (MkInterval lo m)
            (interval_hull (MkInterval lo m) (MkInterval m hi))
            (MkInterval lo hi)
            (interval_inside_hull_left
              (MkInterval lo m)
              (MkInterval m hi))
            (interval_split_hull_inside lo hi m lower upper))
          member

theorem split_upper_member_in_whole
      (lo : Rational) (hi : Rational) (m : Rational) (x : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → IsTrue (interval_contains (MkInterval m hi) x)
      → IsTrue (interval_contains (MkInterval lo hi) x) =
  λlower.
    λupper.
      λmember.
        interval_inside_sound
          (MkInterval m hi)
          (MkInterval lo hi)
          x
          (interval_inside_trans
            (MkInterval m hi)
            (interval_hull (MkInterval lo m) (MkInterval m hi))
            (MkInterval lo hi)
            (interval_inside_hull_right
              (MkInterval lo m)
              (MkInterval m hi))
            (interval_split_hull_inside lo hi m lower upper))
          member

theorem split_hull_inside_interior
    : IsTrue
        (interval_inside
          (interval_hull (MkInterval zero half) (MkInterval half one))
          (MkInterval zero one)) =
  interval_split_hull_inside zero one half Proved Proved

theorem inside_split_hull_exterior
    : IsTrue
        (interval_inside
          (MkInterval zero one)
          (interval_hull (MkInterval zero two) (MkInterval two one))) =
  interval_inside_split_hull zero one two
```

### stdlib/numerical/interval-split-hull-accepts

- spec: `spec/50-stdlib/68-numerical.md` §23.
- promise class: durable invariant.
- given: the shared prefix, with arbitrary open endpoints and split point,
  plus the closed interior and exterior examples.
- expect: the prefix accepts. Both open clients typecheck at the public
  signatures; the two member derivations and both closed applications check.
- measured: the sound open client uses the two ordered premises in lower,
  then upper, order. The complete open client applies with no premises.
- claimed: the half-to-whole inclusion follows from `lo ≤ m ≤ hi`, and
  the whole-to-hull inclusion needs no order premise.
- gap: the closed cases alone do not establish either generic law.
- why: each half's member reaches the whole by `interval_inside_sound`,
  `interval_inside_trans`, a public hull-side inclusion, and the sound law.
  The exterior case checks that the complete direction is not restricted
  to ordered split points.

## Refusal of a false order premise

Append the declaration below to a fresh copy of the shared accepted prefix.
The `m = half` case in that prefix supplies the valid-premise control.

```ken
theorem split_hull_inside_exterior
    : IsTrue
        (interval_inside
          (interval_hull (MkInterval zero two) (MkInterval two one))
          (MkInterval zero one)) =
  interval_split_hull_inside zero one two Proved Proved
```

### stdlib/numerical/interval-split-hull-exterior-premise-refuses

- spec: `spec/50-stdlib/68-numerical.md` §23.
- promise class: durable invariant.
- given: `lo = 0`, `hi = 1`, and exterior `m = 2`, with the two order
  premises supplied using `Proved`.
- expect: the isolated declaration is refused with `KernelRejected` at its
  parsed declaration span while checking
  `IsTrue (rational_leq two one)`. Do not pin diagnostic wording.
- measured: `0 ≤ 2` is true; `2 ≤ 1` is false. The refusal occurs while
  establishing the second premise, before the law is applied.
- claimed: the sound law requires both displayed order premises.
- gap: this refusal does not establish the law's body or direction; it is
  not evidence for the law because the same premise failure occurs without
  any law in play.
- controls: the accepted prefix applies the same law with `m = half` and
  both premises accepted.
- why: the false premise is isolated from the conclusion and from the
  complete law's premise-free behavior.

## Same-form conclusion pair

First, append the interior control to a fresh copy of the accepted prefix:

```ken
theorem split_hull_interior_conclusion
    : IsTrue
        (interval_inside
          (interval_hull (MkInterval zero half) (MkInterval half one))
          (MkInterval zero one)) =
  Proved
```

Then append the exterior declaration below to a fresh copy of the accepted
prefix and the interior control above. Only `m` changes from `half` to
`two`.

```ken
theorem split_hull_exterior_conclusion
    : IsTrue
        (interval_inside
          (interval_hull (MkInterval zero two) (MkInterval two one))
          (MkInterval zero one)) =
  Proved
```

### stdlib/numerical/interval-split-hull-exterior-conclusion-refuses

- spec: `spec/50-stdlib/68-numerical.md` §23.
- promise class: durable invariant.
- given: the same `IsTrue (interval_inside …)` conclusion form at
  `m = half` and `m = two`, each appended to a fresh accepted prefix.
- expect: the interior control accepts. The exterior declaration is refused
  with `KernelRejected` at its parsed declaration span while checking its
  displayed `IsTrue` conclusion. Do not pin diagnostic wording.
- measured: the interior hull is `[0,1]`. The exterior hull is `[0,2]`,
  which is not inside `[0,1]` by endpoint value.
- claimed: the sound conclusion is true for an interior split and false
  for this exterior split.
- gap: these closed values do not prove the generic sound law or its
  ordered-premise boundary.
- controls: both declarations use `Proved` at the same proposition shape;
  only `m` changes. The accepted law-application control is in the shared
  prefix.
- why: unlike the premise refusal above, this pair reaches the conclusion
  itself and shows that a premise-free sound law cannot hold.

The two open clients are the law-specific direction and premise controls.
The premise refusal alone is cause-neutral because it fails before any law
application. The conclusion pair independently separates the true interior
value from the false exterior value.

Architect AC-0 reports the package and D0 checks, including the two refused
rows at their declaration spans. Those are scratch results, not execution of
this seed. No seed fence has been checked as Ken or through the official
conformance runner; no CI, publication, or landing result is claimed.
