# Manhattan metric over natural points

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §2.

This seed is a closed behavioral oracle for the two additional public
laws on `Algorithm.Geometry.NatBox`. It is red until the §2 catalog
proofs and exports are available. The §1 `manhattan_self` theorem is
used only as the premise for a closed instance of the new zero-distance
identity law; it is not counted as a third §2 law. The generic client
below records how a caller uses that law for arbitrary points when
supplied with a zero-distance proof.

The distance values are independently fixed by the coordinates:
`(0,0)` to `(1,1)` is `2`; the legs from `(0,0)` through `(3,2)` to
`(1,1)` are `5` and `3`, totaling `8`. This strict-slack detour
separates the promised `2 ≤ 8` from the unpromised reverse `8 ≤ 2`.
The monotone detour through `(1,1)` to `(2,3)` has leg sum `5`, equal
to the direct distance. Both laws are instantiated below. The finite
examples complement the generic proofs; they do not establish them.

## Shared fixtures and positive assertions

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, manhattan, manhattan_self, manhattan_triangle,
   manhattan_zero_equal)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2

const origin : Point = MkPoint n0 n0
const detour : Point = MkPoint n3 n2
const target : Point = MkPoint n1 n1
const on_path : Point = MkPoint n1 n1
const far : Point = MkPoint n2 n3
const distinct_a : Point = MkPoint n0 n1
const distinct_b : Point = MkPoint n1 n0

theorem direct_distance : Equal Nat (manhattan origin target) n2 = Proved

theorem first_detour_leg :
  Equal Nat (manhattan origin detour) (Suc (Suc (Suc (Suc (Suc n0))))) =
  Proved

theorem second_detour_leg :
  Equal Nat (manhattan detour target) (Suc (Suc (Suc n0))) = Proved

theorem detour_distance :
  Equal Nat (add (manhattan origin detour) (manhattan detour target))
            (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc n0)))))))) = Proved

theorem strict_detour_bounds_direct :
  Equal Bool
    (leq_nat (manhattan origin target)
             (add (manhattan origin detour) (manhattan detour target)))
    True = Proved

theorem strict_detour_does_not_bound_reverse :
  Equal Bool
    (leq_nat (add (manhattan origin detour) (manhattan detour target))
             (manhattan origin target))
    False = Proved

theorem monotone_detour_is_tight :
  Equal Nat (add (manhattan origin on_path) (manhattan on_path far))
            (manhattan origin far) = Proved

theorem distinct_points_are_apart :
  Equal Nat (manhattan distinct_a distinct_b) n2 = Proved

theorem seed_law_triangle :
  IsTrue (leq_nat (manhattan origin target)
                  (add (manhattan origin detour) (manhattan detour target))) =
  manhattan_triangle origin detour target

theorem seed_law_zero_equal :
  Equal Point far far =
  manhattan_zero_equal far far (manhattan_self far)

theorem client_zero_equal (p : Point) (q : Point)
    (h : Equal Nat (manhattan p q) Zero) : Equal Point p q =
  manhattan_zero_equal p q h
```

The distance constants in the direct assertions are fixed fixture values,
not computed by `manhattan`. The theorem instances exercise the public
law results; the generic client has no free assumptions beyond its explicit
zero-distance premise.

## Refusals

Each refusal is considered separately against the unchanged positive
client above. The positive distance `2` is the control for the false
zero-distance assertion. The false reverse bound has its own direct
`False` oracle above, so a rejection cannot be attributed to a malformed
fixture or unavailable operation.

### stdlib/geometry/reject-reverse-manhattan-triangle

- spec: `spec/50-stdlib/67-geometry.md` §2.1.
- promise class: durable invariant.
- given: the strict-slack detour from `origin` through `detour` to
  `target`, with direct distance `2` and detour sum `8`.
- expect: asserting the reverse bound rejects.
- measured: refusal of `8 ≤ 2`, alongside the positive `2 ≤ 8` control.
- claimed: `manhattan_triangle` bounds the direct distance by the sum
  along the detour, not in the reverse direction.
- why: the same three points distinguish the two inequalities.

```ken
-- Reject: the strict-slack detour does not satisfy the reverse bound.
theorem reject_reverse_manhattan_triangle :
  IsTrue (leq_nat
    (add (manhattan origin detour) (manhattan detour target))
    (manhattan origin target)) = Proved
```

### stdlib/geometry/reject-distinct-points-at-zero

- spec: `spec/50-stdlib/67-geometry.md` §2.1.
- promise class: durable invariant.
- given: `distinct_a = (0,1)` and `distinct_b = (1,0)`, whose
  distance is positively checked as `2`.
- expect: asserting their distance is `Zero` rejects.
- measured: refusal of a false zero-distance claim with a positive
  distance control on the same pair.
- claimed: zero distance forces equality of both point coordinates.
- why: the inputs differ on both coordinates and the actual distance is
  nonzero; the positive control rules out a coincidental refusal.

```ken
-- Reject: distinct points do not have zero Manhattan distance.
theorem reject_distinct_points_at_zero :
  Equal Nat (manhattan distinct_a distinct_b) Zero = Proved
```
