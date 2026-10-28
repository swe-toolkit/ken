# Intersection and hull associate on natural boxes

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §7.

This closed seed applies both public Nat Order associativity selectors and
both generic box laws. The three boxes are distinct and pairwise overlap;
their corners differ on both axes. The value checks independently calculate
each side of each box grouping, not by invoking an associativity law. One
false mixed-operation equality is attempted only after a fresh, complete
copy of the accepted client. Closed instances cannot prove the four generic
laws. The earlier spec-and-seed candidate did not deliver the §7 catalog
exports. The separate Foundation catalog candidate now exports the four
checked laws and runs this seed in a package test; that test is not official
conformance-runner execution, CI, publication or landing.

## Accepted client

`box_a` spans `(0,1)–(4,5)`, `box_b` spans `(2,0)–(6,3)`, and
`box_c` spans `(1,2)–(5,6)`. The intersection of all three spans
`(2,2)–(4,3)`; their hull spans `(0,0)–(6,6)`. The closed `Equal Box`
value controls use `Refl`, as do the same-form controls for the two
mixed-operation sides. Nat value controls use `Proved`.

```ken
import Data.Numeric.Nat.Order (min, max)

import Algorithm.Geometry.NatBox
  (Point, MkPoint, Box, MkBox, intersect, hull, intersect_assoc, hull_assoc)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const box_a : Box = MkBox (MkPoint Zero one) (MkPoint four five)

const box_b : Box = MkBox (MkPoint two Zero) (MkPoint six three)

const box_c : Box = MkBox (MkPoint one two) (MkPoint five six)

const intersection_result : Box = MkBox (MkPoint two two) (MkPoint four three)

const hull_result : Box = MkBox (MkPoint Zero Zero) (MkPoint six six)

const mixed_left_result : Box = MkBox (MkPoint one two) (MkPoint five five)

const mixed_right_result : Box = MkBox (MkPoint Zero one) (MkPoint five five)

theorem min_value : Equal Nat (min (min two Zero) one) Zero = Proved

theorem max_value : Equal Nat (max (max two Zero) one) two = Proved

theorem min_regrouped : Equal Nat (min (min two Zero) one) (min two (min Zero one)) =
  (proof assoc for min) two Zero one

theorem max_regrouped : Equal Nat (max (max two Zero) one) (max two (max Zero one)) =
  (proof assoc for max) two Zero one

theorem intersect_left_value
    : Equal Box (intersect (intersect box_a box_b) box_c) intersection_result =
  Refl

theorem intersect_right_value
    : Equal Box (intersect box_a (intersect box_b box_c)) intersection_result =
  Refl

theorem hull_left_value : Equal Box (hull (hull box_a box_b) box_c) hull_result = Refl

theorem hull_right_value : Equal Box (hull box_a (hull box_b box_c)) hull_result = Refl

theorem intersect_regrouped
    : Equal Box
        (intersect (intersect box_a box_b) box_c)
        (intersect box_a (intersect box_b box_c)) =
  intersect_assoc box_a box_b box_c

theorem hull_regrouped
    : Equal Box (hull (hull box_a box_b) box_c) (hull box_a (hull box_b box_c)) =
  hull_assoc box_a box_b box_c

theorem intersect_regrouped_closed
    : Equal Box
        (intersect (intersect box_a box_b) box_c)
        (intersect box_a (intersect box_b box_c)) =
  Refl

theorem hull_regrouped_closed
    : Equal Box (hull (hull box_a box_b) box_c) (hull box_a (hull box_b box_c)) =
  Refl

theorem mixed_left_value : Equal Box (intersect (hull box_a box_b) box_c) mixed_left_result =
  Refl

theorem mixed_right_value : Equal Box (hull box_a (intersect box_b box_c)) mixed_right_result =
  Refl
```

### `stdlib/geometry/nat-extrema-associate`

- spec: §7.1.
- promise class: durable invariant.
- given: natural coordinates `two`, `Zero`, and `one`.
- expect: `(proof assoc for min)` and `(proof assoc for max)` are
  publicly selectable and prove the stated regrouped extrema.
- measured: two direct selector applications and independent closed
  `min_value`/`max_value` observations.
- claimed: both selectors are public checked laws for all natural
  arguments, not only this closed triple.
- gap: closed applications do not establish generic ownership,
  transparency or a shipped catalog export.
- why: a box-only private derivation would not expose the public Nat
  associativity selectors that §7 requires.

### `stdlib/geometry/box-intersection-associates`

- spec: §7.2.
- promise class: durable invariant.
- given: the three distinct pairwise overlapping boxes above.
- expect: `intersect_assoc` proves the structural equality of the
  two groupings.
- measured: the public-law application and independently computed
  left and right corner results, each `(2,2)–(4,3)`.
- claimed: the law holds for every three boxes, including inverted ones.
- gap: this one input does not establish the generic proof or cover
  inverted bounds.
- why: equal stored coordinates, not just equal member sets, make
  this an `Equal Box` law.

### `stdlib/geometry/box-hull-associates`

- spec: §7.2.
- promise class: durable invariant.
- given: the same three boxes.
- expect: `hull_assoc` proves the two groupings structurally equal.
- measured: the public-law application and independent left and right
  corner results, each `(0,0)–(6,6)`.
- claimed: the law is unconditional for every box orientation.
- gap: closed results cannot prove a quantified generic theorem.
- why: the matching corner extremes justify a structural equality
  without a new quotient or membership interpretation.

## One isolated refused claim

Begin a **fresh copy of the entire accepted client**, including both
selectors, all four independent grouping-value observations, both law
applications and the two independently computed mixed sides. Append only
this theorem. The left mixed side is `(1,2)–(5,5)`, while the right mixed
side is `(0,1)–(5,5)` on the same three boxes. Expect refusal **inside**
the appended claim, not at an import or a preceding declaration. The
accepted `mixed_left_value` and `mixed_right_value` use the same closed
`Equal Box ... = Refl` form with true propositions; the false proposition
cannot be dismissed as an unsuitable proof form.

### `stdlib/geometry/reject-mixed-intersection-hull-regrouping`

- spec: §7.2.
- promise class: durable invariant.
- given: the same three boxes and independent mixed-side values.
- expect: the sole appended `seed_neg` is refused at its false equality.
- measured: the same-input true controls accept with `Refl`, and the
  two closed mixed sides have distinct lower corners.
- claimed: individual associativity of intersection and hull does not
  make the mixed regrouping an equality.
- gap: this refusal cannot establish the generic positive theorems;
  those require separate checked bodies.
- why: a wrong equation conflating the two operations would change
  an observable stored corner even on overlapping boxes.

```ken
theorem seed_neg
    : Equal Box (intersect (hull box_a box_b) box_c) (hull box_a (intersect box_b box_c)) =
  Refl
```

## Evidence boundary

Architect's isolated D0 on `372a6a1a6` applied only a checked
Order/NatBox code patch (SHA-256
`91f7c910d2058f04ec5544aedcfd1e16dd33dc62532f7aab8fae1adaadfbc49e`).
Both packages passed `ken check` and `ken fmt --check`; the four generic
proofs were owned Transparent exports, no new `Axiom` was introduced,
and scratch `trusted_base()` GlobalId identity sets stayed 114 → 114.
The Architect-reported probe accepted the three-box value and law
controls and rejected the false mixed equation with `TypeMismatch`.
Its probe was a D0 scratch client, not this authored fence or a shipped
§7 export. The author separately overlaid **only the catalog paths**
of that D0 patch on a disposable worktree at `372a6a1a6`, without
including its scratch Rust test. Both packages passed `ken check` and
`ken fmt --check` (rc 0). The entire authored accepted fence passed
`ken check` and `ken fmt --check` (rc 0); a fresh full accepted prefix
plus the sole `seed_neg` passed `ken fmt --check` and failed
`ken check` rc 1/E0038 `TypeMismatch`, at the appended `Refl`
bytes 2544–2548. On untouched base, removing only the
four absent law applications and absent law imports left ten independent
value and true-equality controls, which passed rc 0. The full client
on that base failed rc 1/E0012 `UnboundName` at the missing
`intersect_assoc` import. Logs under `/workspaces/ken/local/` have the prefix
`spec-author-box-assoc-` and suffixes `overlay-order-check`,
`overlay-order-fmt`, `overlay-natbox-check`, `overlay-natbox-fmt`,
`overlay-positive`, `overlay-negative`, `seed-fmt`, `negative-fmt`,
`base-values`, and `base-laws` (each ending in `.log`).
The reused local CLI was not independently built at the detached base.
These historical checks use a **D0 overlay**, not a shipped catalog
export. The earlier spec-and-seed candidate's base did not include the
separate Foundation catalog addition.

The Foundation catalog candidate now exports the four checked laws and
runs this seed via `cat_geometry_box_intersect_hull_associativity`. Its
package test root-loads both providers and all four generic laws, then
checks the complete accepted client and refuses the isolated `seed_neg`
at its own false proposition. Exact trusted-GlobalId identity sets and
targeted ambient rows are candidate-local measurements, not a full-corpus
census or landed trust. This package test is not official
conformance-runner execution, CI, publication or landing.
