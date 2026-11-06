# Box inclusion agrees with intersection and hull

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §16.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INSIDE-INTERSECT-HULL.md`.

These cases exercise the ruled public laws on closed natural-coordinate
boxes. The accepted client applies both laws to `small` inside `big`, and
the intersection law to `small` inside `tall`. The refused claim uses the
same `Refl` form for `small` and `short`. `tall` and `short` differ only
in their high y coordinate. Closed examples are contract vectors, not
proofs of the generic laws.

The cases state expected behavior; CV has run no semantic Ken tests or
official conformance runner for this seed.

## Accepted client

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.Geometry.NatBox
  (Box, MkPoint, MkBox, inside, intersect, hull,
   intersect_eq_of_inside, hull_eq_of_inside)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two

const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const big : Box = MkBox (MkPoint zero zero) (MkPoint three three)
const tall : Box = MkBox (MkPoint zero zero) (MkPoint three two)
const short : Box = MkBox (MkPoint zero zero) (MkPoint three one)

theorem small_inside_big : IsTrue (inside small big) = Proved

theorem intersect_small_big
    : Equal Box (intersect small big) small =
  intersect_eq_of_inside small big small_inside_big

theorem hull_small_big
    : Equal Box (hull small big) big =
  hull_eq_of_inside small big small_inside_big

theorem small_inside_tall : IsTrue (inside small tall) = Proved

theorem intersect_small_tall
    : Equal Box (intersect small tall) small =
  intersect_eq_of_inside small tall small_inside_tall

theorem small_not_inside_short
    : Equal Bool (inside small short) False =
  Proved

theorem intersect_small_tall_same_form
    : Equal Box (intersect small tall) small = Refl
```

### stdlib/geometry/box-inside-intersect-and-hull

- spec: §16.
- promise class: durable invariant.
- given: the closed boxes `small=(1,1)–(2,2)` and
  `big=(0,0)–(3,3)`, with `inside small big` proved.
- expect: the public intersection law proves
  `Equal Box (intersect small big) small`; the public hull law proves
  `Equal Box (hull small big) big`.
- why: both laws are applied with the same `inside` premise. These closed
  applications illustrate the public contract but do not establish the
  generic theorems.

### stdlib/geometry/box-inside-intersect-tall

- spec: §16.
- promise class: durable invariant.
- given: `small=(1,1)–(2,2)` inside `tall=(0,0)–(3,2)`.
- expect: the public intersection law proves
  `Equal Box (intersect small tall) small`; the same-form `Refl` control
  in the accepted client also accepts.
- why: the upper y bound is equal to `small`'s upper y bound, so the
  intersection returns the stored `small` box.

## Isolated refusal

Start from a fresh copy of the accepted client above and append only this
claim. Expect rejection at its `Refl`, not at an import, missing law,
malformed fixture or earlier declaration.

### stdlib/geometry/reject-intersect-small-short-equals-small

- spec: §16.
- promise class: durable invariant.
- given: the same `small` box and the same-form accepted control for
  `tall`; `short=(0,0)–(3,1)` differs from `tall` only in its high y
  coordinate, and `inside small short` is `False`.
- expect: reject the claim that `intersect small short` is structurally
  equal to `small` at the appended `Refl`.
- why: the intersection's high y coordinate is `1`, not `small`'s `2`.
  The accepted `tall` control holds the expression and proof form fixed;
  only the outer high y coordinate changes.

```ken
theorem intersect_small_short_refused
    : Equal Box (intersect small short) small = Refl
```

## Evidence boundary

This seed encodes the ruled §16 contract. CV has not run the source through
`ken check`, semantic tests, or the official conformance runner. The D0
measurements reported by the Architect and Foundation implementer are
attributed to them and their scratch branch; they do not establish behavior
on this seed, the implementation candidate, CI, publication or landing.
