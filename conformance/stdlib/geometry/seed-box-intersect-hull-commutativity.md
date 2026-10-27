# Intersection and hull commute on natural boxes

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §6.

This seed exercises the two ruled public `NatBox` laws on overlapping,
unequal boxes **and** both public `Data.Numeric.Nat.Order` selectors.
The catalog candidate's package test roots-loads those four checked
exports and exercises this authored client. The seed alone does not
prove the generic statements. Independent Nat and Box value observations
ground the swaps. The sole refusal appends a false Box equality to a
fresh copy of the complete accepted client. This package-test execution
is not official conformance-runner execution or a landing claim.

## Accepted client

The first box spans `(0,0)–(2,2)`; the second spans `(1,1)–(3,3)`.
Their intersection spans `(1,1)–(2,2)`, whereas their hull spans
`(0,0)–(3,3)`. The two boxes overlap and neither equals the other.
The distinct natural operands `one` and `two` give `min one two = one`
and `max one two = two`. Closed `Equal Box` goals use `Refl`;
replacing that proof with `Proved` is not a valid control for this
carrier.

```ken
import Data.Numeric.Nat.Order (min, max)

import Algorithm.Geometry.NatBox
  (Point, MkPoint, Box, MkBox, intersect, hull, intersect_comm, hull_comm)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const b02 : Box = MkBox (MkPoint Zero Zero) (MkPoint two two)
const b13 : Box = MkBox (MkPoint one one) (MkPoint three three)

theorem min_value : Equal Nat (min one two) one = Proved

theorem max_value : Equal Nat (max one two) two = Proved

theorem min_swapped : Equal Nat (min one two) (min two one) =
  (proof comm for min) one two

theorem max_swapped : Equal Nat (max one two) (max two one) =
  (proof comm for max) one two

theorem intersect_value
    : Equal Box (intersect b02 b13) (MkBox (MkPoint one one) (MkPoint two two)) =
  Refl

theorem hull_value
    : Equal Box (hull b02 b13) (MkBox (MkPoint Zero Zero) (MkPoint three three)) =
  Refl

theorem intersect_swapped : Equal Box (intersect b02 b13) (intersect b13 b02) =
  intersect_comm b02 b13

theorem hull_swapped : Equal Box (hull b02 b13) (hull b13 b02) = hull_comm b02 b13

theorem hull_swapped_closed : Equal Box (hull b02 b13) (hull b13 b02) = Refl
```

### `stdlib/geometry/nat-extrema-commute`

- spec: §6.1.
- promise class: durable invariant.
- given: distinct natural operands `one` and `two`.
- expect: selectively importing `min` and `max` makes their public
  `(proof comm for min)` and `(proof comm for max)` selectors available;
  both applications have the ruled `Equal Nat` result types.
- measured: `min_swapped` and `max_swapped` apply the selectors to
  concrete operands; independent `min_value` and `max_value` check
  distinct closed extrema without using either selector.
- claimed: both selector laws hold for all `Nat` arguments and are
  public from `Data.Numeric.Nat.Order`.
- gap: two closed instances do not prove either generic law or its
  ownership/visibility on shipped exports until run against them.
- why: Box laws could be checked using private helpers while both
  public selector exports were absent or misstated; these direct
  imports and applications would then fail.

### `stdlib/geometry/box-intersection-commutes`

- spec: §6.2.
- promise class: durable invariant.
- given: the overlapping unequal `b02` and `b13` boxes.
- expect: `intersect_comm b02 b13` proves the two operand orders
  structurally equal as `Box` values.
- measured: `intersect_value` independently computes the stored
  corners `(1,1)–(2,2)`; `intersect_swapped` applies the public law.
- claimed: the public law quantifies over all boxes, including
  inverted boxes, not just this pair.
- gap: the closed pair does not establish the generic proof or
  inspect all possible input orientations.
- why: swapped Nat selectors preserve each corner component, rather
  than merely the set of contained points.

### `stdlib/geometry/box-hull-commutes`

- spec: §6.2.
- promise class: durable invariant.
- given: the same overlapping unequal `b02` and `b13` boxes.
- expect: `hull_comm b02 b13` proves the two operand orders
  structurally equal as `Box` values.
- measured: `hull_value` independently computes corners
  `(0,0)–(3,3)`; `hull_swapped` applies the public law, while
  `hull_swapped_closed` checks a true closed equality with `Refl`.
- claimed: the hull equality is generic, independent of validity or
  nonemptiness premises.
- gap: the closed instance cannot prove the quantified law and does
  not measure a trusted-base delta.
- why: the same-operand closed control separates the false statement
  below from a merely unsuitable `Refl` proof form.

## One isolated refused claim

For this case, start from a **fresh copy of the entire accepted client**,
including both Nat selector applications, the two Box-value controls
and `hull_swapped_closed`, then append **only** the following theorem.
Expect refusal inside the appended claim, not at its import, fixture
or any accepted declaration. Its
`hull b02 b13` side has corners `(0,0)–(3,3)` and its
`intersect b13 b02` side has corners `(1,1)–(2,2)`; swapping operands
cannot make their different bounds equal. The accepted control has
the same `Equal Box ... = Refl` proof form, but equal sides.

### `stdlib/geometry/reject-hull-equals-intersection`

- spec: §6.2.
- promise class: durable invariant.
- given: the same `b02`/`b13` input pair and the independently
  accepted hull and intersection corner values.
- expect: `seed_neg` is refused at its false structural equality.
- measured: the full accepted prefix, including the two public Order
  selector applications, is checked before appending the one false
  claim; `hull_swapped_closed` accepts with `Refl` on the same
  operands and carrier.
- claimed: commutativity of either operation does not identify the
  hull of a pair with its intersection.
- gap: the refusal does not establish the two general positive
  theorems; the generic bodies require their own kernel checks.
- why: the compared closed boxes have different stored corners,
  irrespective of point-set or interval-endpoint equivalence.

```ken
theorem seed_neg : Equal Box (hull b02 b13) (intersect b13 b02) = Refl
```

## Evidence boundary

Architect D0 on `0bb872591`, with only its checked Order/NatBox
package patch, reported `ken check` and `ken fmt --check` rc 0 on both
packages, two probe tests passing, no new `Axiom`, and identical
scratch `trusted_base()` GlobalId sets 114 → 114. Its separately
assembled **earlier Box-only client** accepted and refused the
appended `seed_neg` with `TypeMismatch` at its false closed `Equal Box`
claim; that client did **not** test the public Order selectors. These
are isolated D0 measurements, not checks against shipped §6 exports.

The author checked these exact fences separately in an isolated
worktree at `0bb872591`, with only the two catalog paths of Architect's
D0 patch overlaid (patch SHA-256
`f013faa9c92fb7bd0a049dfeffdeda39ad87c9796fdfa31713f07923fa9ef1b1`).
The locally built scratch CLI had SHA-256
`645e6ddf67dbb45697ad1ba49db8b975a745d94520d2d1c1c158b7edd673b256`.
The revised full positive block, now including two Order selector
imports and both direct applications, returned rc 0. A fresh full
positive prefix plus only `seed_neg` returned rc 1/E0038
`TypeMismatch`, `Refl expects an Eq-shaped goal`, at bytes 1268–1272
inside the appended theorem starting at byte 1202. Logs are saved
under `/workspaces/ken/local/spec-author/box-comm-d0/` as
`spec-author-box-comm-{RespinPositive,RespinFalseEquality}.log`;
the positive log is empty because rc 0 produced no diagnostic.
These are historical **D0-overlay** checks, not tests of the catalog
candidate. The candidate's package test roots-loads Order and NatBox,
checks the entire accepted seed client including `nat-extrema-commute`
and the same-form `hull_swapped_closed` control, and pins `seed_neg` to
the false hull-versus-intersection proposition on the same pair before
observing `TypeMismatch` at its own `Refl`. A separate generic client
consumes the four public checked types; selective imports of the three
private helpers are refused, and exact `trusted_base()` GlobalId sets
agree before and after loading (114 → 114). This is **local package-test
evidence**, not official conformance-runner execution, CI, publication,
landing or landed trust.
