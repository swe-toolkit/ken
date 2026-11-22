# Rational interval split covers

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §21.

These cases exercise the public function `interval_split_covers` on two
members of `[0,1]` split at `1/2`. Its result is a Type-sorted `Or`,
so the checked law is a `pub fn`, not a theorem, and clients can
eliminate it. The accepted cases eliminate each result to the half that
contains the member. The refusal uses the same application at `3/4` but
claims the lower half.

The law has no premise on the split point. These midpoint instances do
not cover exterior split points or prove the generic law. Architect's
AC-0 scratch measurements ground these forms; this seed has not been
executed as Ken or by an official conformance runner.

## Shared accepted prefix

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Core.Logic.Or (Or, Inl, Inr)
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_split_covers)

const zero : Rational = MkRational Zero Zero
const one : Rational = MkRational (Suc Zero) Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const quarter : Rational =
  MkRational (Suc Zero) (Suc (Suc (Suc Zero)))
const three_quarters : Rational =
  MkRational (Suc (Suc (Suc Zero))) (Suc (Suc (Suc Zero)))

const quarter_split
    : Or
        (IsTrue (interval_contains (MkInterval zero half) quarter))
        (IsTrue (interval_contains (MkInterval half one) quarter)) =
  interval_split_covers zero one half quarter Proved

theorem quarter_lower :
  IsTrue (interval_contains (MkInterval zero half) quarter) =
  match interval_split_covers zero one half quarter Proved {
    Inl below ↦ below;
    Inr above ↦ Proved
  }

theorem three_quarters_upper :
  IsTrue (interval_contains (MkInterval half one) three_quarters) =
  match interval_split_covers zero one half three_quarters Proved {
    Inl below ↦ Proved;
    Inr above ↦ above
  }

fn split_open_swap (lo : Rational) (hi : Rational) (m : Rational)
      (x : Rational)
    : IsTrue (interval_contains (MkInterval lo hi) x)
      → Or
        (IsTrue (interval_contains (MkInterval m hi) x))
        (IsTrue (interval_contains (MkInterval lo m) x)) =
  λmember.
    match interval_split_covers lo hi m x member {
      Inl below ↦
        Inr
          (IsTrue (interval_contains (MkInterval m hi) x))
          (IsTrue (interval_contains (MkInterval lo m) x))
          below;
      Inr above ↦
        Inl
          (IsTrue (interval_contains (MkInterval m hi) x))
          (IsTrue (interval_contains (MkInterval lo m) x))
          above
    }
```

### stdlib/numerical/interval-split-covers-quarter-result

- spec: `spec/50-stdlib/68-numerical.md` §21.
- promise class: durable invariant.
- given: the law applied to `[0,1]`, split at `1/2`, for `1/4`.
- expect: the closed `quarter_split` client accepts with the law's `Or`
  result type; `quarter_lower` eliminates it to membership in `[0,1/2]`.
- measured: the original premise is `Proved`; the lower membership is
  checked by the result type of `quarter_lower`.
- claimed: a member below the split is carried by the lower disjunct.
- gap: this closed instance does not prove the generic law.
- why: `quarter` is `1/4`, strictly below `1/2`, and the selected
  lower interval is the one claimed.

### stdlib/numerical/interval-split-covers-three-quarters-upper

- spec: `spec/50-stdlib/68-numerical.md` §21.
- promise class: durable invariant.
- given: the same interval and split, for `3/4`.
- expect: `three_quarters_upper` accepts after eliminating the law's
  `Or` to membership in `[1/2,1]`.
- measured: the original premise is `Proved`; the upper membership is
  checked by the result type of `three_quarters_upper`.
- claimed: a member above the split is carried by the upper disjunct.
- gap: this closed instance does not prove the generic law.
- why: `three_quarters` is `3/4`, strictly above `1/2`; the positive
  case fixes the same law application used by the refusal below.

### stdlib/numerical/interval-split-covers-open-or-elimination

- spec: `spec/50-stdlib/68-numerical.md` §21.
- promise class: durable invariant.
- given: arbitrary `lo`, `hi`, `m`, and `x`, with only the original
  membership premise.
- expect: `split_open_swap` accepts and eliminates the public `Or`,
  swapping its two disjuncts for the client result.
- claimed: a client can consume both branch payloads of the public law.
- gap: this client does not prove the law for arbitrary inputs.
- why: unlike a closed call, this uses the exact generic public function
  and typechecks both `Inl` and `Inr` conversions.

## Refusal on a fresh accepted prefix

Append the following declaration alone to a fresh copy of the accepted
prefix above. The full-union applications and both correct-half controls
must be accepted before this refusal is checked.

```ken
-- Refuse: three-quarters is above the split, not in the lower half.
theorem three_quarters_lower_by_law
    : IsTrue (interval_contains (MkInterval zero half) three_quarters) =
  match interval_split_covers zero one half three_quarters Proved {
    Inl below ↦ below;
    Inr above ↦ above
  }
```

### stdlib/numerical/interval-split-covers-three-quarters-lower-refused

- spec: `spec/50-stdlib/68-numerical.md` §21.
- promise class: durable invariant.
- given: the same `[0,1]` interval, split point, member, premise, and
  law application as `three_quarters_upper`, but claiming lower-half
  membership.
- expect: the isolated declaration is refused with
  `KernelRejected(TypeMismatch)` at declaration level; its reported
  span is within the declaration. Do not pin an exact token span.
- measured: `Inr` carries upper-half membership, while the goal asks
  for lower-half membership.
- claimed: upper-half evidence cannot establish the lower-half claim.
- gap: this one false lower-half claim does not characterize every
  incorrect partition.
- why: the positive upper control accepts for the same input and law
  application; the refusal changes only the claimed half. Replacing the
  upper payload with lower-half evidence would make this false claim
  accept and make the upper control reject. This is not a refusal by
  closed computation of `Proved`.

The accepted and refused declarations are independent source cases. The
Architect's AC-0 scratch checked the D0 forms, not this committed seed;
those measurements do not establish seed execution, official
conformance, CI, publication, or landing.
