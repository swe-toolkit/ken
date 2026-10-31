# A box containing a point contains its stored corners

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §11.

This seed pins the two public corner-membership laws on a closed client
fixture and keeps their witness premise observable. Finite cases do not
replace the generic checked theorems.

The cases below are authored expectations. This seed has not been
Ken-checked or run by the official conformance runner. No CI,
publication, landing, or landed-trust result is claimed here.

Provenance: expected behavior is derived from §11, AC-2, and closed
natural-number arithmetic. I inspected the candidate-local package test
to confirm public names and the assigned fixture; I did not run it or
treat it as conformance evidence. No external or `local/refs/`
implementation was consulted. No copyleft material informed this seed,
so no originality scan was run.

## Shared positive client

The box has lower corner `(1,1)`, upper corner `(4,3)`, and contained
witness `(2,2)`. The value checks compute the witness and each corner's
membership directly. The final two declarations separately apply the
low- and high-corner laws using the witness proof.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.Geometry.NatBox
  (Point, Box, MkPoint, MkBox, contains,
   contains_lo_corner, contains_hi_corner)

const n1 : Nat = Suc Zero
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3

const lower : Point = MkPoint n1 n1
const upper : Point = MkPoint n4 n3
const interior : Point = MkPoint n2 n2
const bounds : Box = MkBox lower upper

theorem witness_is_inside :
  IsTrue (contains bounds interior) = Proved

theorem lower_corner_value :
  Equal Bool (contains bounds lower) True = Proved

theorem upper_corner_value :
  Equal Bool (contains bounds upper) True = Proved

theorem lower_corner_from_witness :
  IsTrue (contains bounds lower) =
  contains_lo_corner lower upper interior Proved

theorem upper_corner_from_witness :
  IsTrue (contains bounds upper) =
  contains_hi_corner lower upper interior Proved
```

### stdlib/geometry/box-corners-from-contained-witness

- spec: `spec/50-stdlib/67-geometry.md` §11.
- promise class: durable invariant.
- given: `bounds=(1,1)–(4,3)` and `interior=(2,2)`.
- expect: the witness is accepted; each corner's direct membership
  computes `True`; each public law applies to that same witness.
- measured: separate Boolean results for the lower and upper corners,
  plus separate applications of `contains_lo_corner` and
  `contains_hi_corner` with a checked witness premise.
- claimed: any point in a box entails membership of both stored
  corners in that box.
- gap: these closed values do not establish the universal laws; the
  checked theorem bodies carry that quantification.
- why: the non-corner witness makes the premise distinct from either
  conclusion, and the two law applications pin both directions.

## Refuse the own-lower-corner claim for an inverted box

Append this block alone to the shared positive client above. The first
declaration is an accepted value control; the final declaration is the
isolated refusal target.

### stdlib/geometry/reject-inverted-box-own-lower-corner

- spec: `spec/50-stdlib/67-geometry.md` §11.
- promise class: durable invariant.
- given: the inverted box `(3,3)–(1,1)` and its stored lower corner.
- expect: the control computes `False`; the claim that the box contains
  that corner is refused at its own theorem.
- measured: the target proposition is `IsTrue` of the same false
  membership established by the preceding control.
- claimed: a corner-membership conclusion is not available without a
  contained-point witness.
- gap: this one inverted box does not characterize every empty box.
- why: both upper-bound comparisons ask `3 ≤ 1`. The accepted
  `False` control anchors the refusal to membership, not malformed
  syntax.

```ken
-- Accepted control: the inverted box excludes its stored lower corner.
theorem inverted_lower_corner_is_outside :
  Equal Bool
    (contains
      (MkBox (MkPoint n3 n3) (MkPoint n1 n1))
      (MkPoint n3 n3))
    False = Proved

-- Refuse: this claim omits the law's required witness premise.
theorem reject_inverted_own_lower_corner :
  IsTrue
    (contains
      (MkBox (MkPoint n3 n3) (MkPoint n1 n1))
      (MkPoint n3 n3)) = Proved
```
