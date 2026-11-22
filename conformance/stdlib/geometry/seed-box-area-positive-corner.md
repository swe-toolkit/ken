# Positive area supplies a stored corner as a member

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §20, child
`8230ad08ab5b14dc744c142fb26914fb21e11742`.
Architect AC-0: `evt_74t8r7pkyncdg`.

These authored expectations follow §20's exact public theorem and its
closed boundary pair. The seed has not been executed. Architect D0's
refusal was observed at theorem-declaration granularity; this seed does
not claim a finer span, official conformance, CI, publication, landing,
or landed trust.

## Accepted positive client

The box from `(1,2)` to `(3,2)` has inclusive area three. Its low corner
is a member, and the public law applies from the positive-area premise.
The high-corner result is derived by applying the existing §11 law to
the low-corner member; §20 adds no second export.

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)
import Algorithm.Geometry.NatBox
  (Point, Box, MkPoint, MkBox, area, contains,
   contains_hi_corner, area_positive_contains_lo_corner)

const n1 : Nat = Suc Zero
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2

const positive_low : Point = MkPoint n1 n2
const positive_high : Point = MkPoint n3 n2
const positive_box : Box = MkBox positive_low positive_high

const inverted_low : Point = MkPoint n1 n2
const inverted_high : Point = MkPoint n3 n1
const y_inverted_box : Box = MkBox inverted_low inverted_high

theorem positive_area_value :
  Equal Nat (area positive_box) n3 = Proved

theorem positive_area_premise :
  IsTrue (leq_nat n1 (area positive_box)) = Proved

theorem positive_low_corner_value :
  Equal Bool (contains positive_box positive_low) True = Proved

theorem positive_low_corner_from_area :
  IsTrue (contains positive_box positive_low) =
  area_positive_contains_lo_corner positive_low positive_high Proved

theorem positive_high_corner_value :
  Equal Bool (contains positive_box positive_high) True = Proved

theorem positive_high_corner_from_low :
  IsTrue (contains positive_box positive_high) =
  contains_hi_corner positive_low positive_high positive_low
    (area_positive_contains_lo_corner positive_low positive_high Proved)

theorem y_inverted_area_value :
  Equal Nat (area y_inverted_box) Zero = Proved

theorem y_inverted_positive_area_premise_false :
  Equal Bool (leq_nat n1 (area y_inverted_box)) False = Proved

theorem y_inverted_low_corner_false :
  Equal Bool (contains y_inverted_box inverted_low) False = Proved
```

### stdlib/geometry/box-area-positive-low-corner

- spec: §20, positive area implies membership of the stored low corner
- promise class: normative compatibility vector
- given: `positive_box=(1,2)–(3,2)` and
  `y_inverted_box=(1,2)–(3,1)`
- expect: the first area is 3, its low corner is contained, and the public
  law applies; the inverted box has area 0, a false positive-area premise,
  and no membership of its low corner
- measured: when run, direct area and membership values, the premise,
  the public low-corner law, and the existing high-corner composition
- claimed: the theorem uses exactly the positive inclusive-area premise;
  the high corner follows through the existing §11 law
- gap: these closed boxes do not establish the generic theorem
- controls: the y-inverted case separately asserts area 0, the false
  premise, and false low-corner membership
- why: changing only the upper y coordinate from 2 to 1 crosses the
  positive-area and membership boundaries while keeping the x axis valid

## Refuse the theorem on the y-inverted box

Start from a fresh copy of the complete accepted client above, including
its area-0, false-premise, and false-membership controls. Append only the
same-form theorem claim below. The box has no positive-area witness: its
y span is zero, so `Proved` cannot discharge the law's premise.

```ken
theorem reject_y_inverted_low_corner_claim :
  IsTrue (contains y_inverted_box inverted_low) =
  area_positive_contains_lo_corner inverted_low inverted_high Proved
```

### stdlib/geometry/reject-area-positive-low-corner-y-inverted

- spec: §20, the single positive-area premise is required
- promise class: durable invariant
- given: the fresh accepted prefix above, followed only by
  `reject_y_inverted_low_corner_claim`
- expect: the false claim is refused with `TypeMismatch` at
  theorem-declaration granularity; do not pin a finer span
- measured: Architect D0 observed declaration-level `TypeMismatch` for
  this y-inverted same-form refusal; this seed remains unexecuted
- claimed: the low-corner theorem cannot be applied when the box's area
  is zero
- gap: this one inverted box does not characterize every zero-area box
- controls: accepted positive-area theorem application and separate
  y-inverted area, premise, and membership values
- why: the accepted box checks the public law; the fresh refusal supplies
  `Proved` for the explicitly false area premise

## Evidence boundary

The expected values are grounded in §§1.1, 4.1, 11, 15 and 20 of the
exact Spec child. Architect AC-0 is scratch design evidence, not execution
of this seed or an official conformance result.
