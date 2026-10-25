# Corner-wise order of boxes

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §5.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INSIDE-ORDER.md`.

This authored seed instantiates the three public §5 laws on closed
natural-coordinate boxes. The §5 catalog exports are absent at its base
`4f337ec26eaebf62d4951878e96809619aedc25f`. Author `ken check`
results below use an isolated overlay of Architect's ruled D0 patch, not
the shipped package or official conformance runner. A closed instance
is not a substitute for a generic proof. Every case promises a
**durable invariant**, including both reverse-inclusion refusals.

## Shared fixtures and accepted client

The five boxes are `small=(1,1)–(2,2)`, `mid=(0,0)–(3,3)`,
`big=(0,0)–(4,4)`, `wide=(1,0)–(4,2)` and inverted
`(3,1)–(1,2)`. The inverted box has no points, but its stored corners
are inside both `mid` and `big`. `small` is inside `mid` and `wide`.
The `mid`/`wide` intersection is `(1,0)–(3,2)`; its closed `Equal Box`
equation uses `Refl`, not an assumed point-set equality.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.Geometry.NatBox
  (MkPoint, Box, MkBox, inside, intersect, hull,
   inside_trans, intersect_greatest, hull_least)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three

const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const mid : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const big : Box = MkBox (MkPoint Zero Zero) (MkPoint four four)
const wide : Box = MkBox (MkPoint one Zero) (MkPoint four two)
const inverted : Box = MkBox (MkPoint three one) (MkPoint one two)

theorem inverted_inside_mid : Equal Bool (inside inverted mid) True =
  Proved

theorem mid_inside_big : Equal Bool (inside mid big) True =
  Proved

theorem inverted_chains_to_big : IsTrue (inside inverted big) =
  inside_trans inverted mid big Proved Proved

theorem inverted_inside_big_value : Equal Bool (inside inverted big) True =
  Proved

theorem small_inside_mid : Equal Bool (inside small mid) True =
  Proved

theorem small_inside_wide : Equal Bool (inside small wide) True =
  Proved

theorem small_inside_meet : IsTrue (inside small (intersect mid wide)) =
  intersect_greatest small mid wide Proved Proved

theorem meet_corners
    : Equal Box (intersect mid wide)
        (MkBox (MkPoint one Zero) (MkPoint three two)) =
  Refl

theorem join_with_inverted_inside_mid
    : IsTrue (inside (hull small inverted) mid) =
  hull_least small inverted mid Proved Proved

theorem hull_of_reverse_is_mid : IsTrue (inside (hull small mid) mid) =
  Proved

theorem mid_not_inside_small : Equal Bool (inside mid small) False =
  Proved

theorem meet_of_reverse_is_small
    : IsTrue (inside (intersect mid small) small) =
  Proved

theorem big_not_inside_inverted
    : Equal Bool (inside big inverted) False =
  Proved
```

The direct `Equal Bool` observations control the premises and the
reverse claims separately from applications of the three new laws.
`inside_trans` starts with an inverted box; `intersect_greatest`
uses a nontrivial overlapping pair; `hull_least` uses an inverted
operand. The hull and intersection positive controls share the exact
boxes and directions of the corresponding refused claims below.

### stdlib/geometry/box-inside-order-transitive-inverted

- spec: §5.1.
- promise class: durable invariant.
- given: inverted box inside `mid`, and `mid` inside `big`.
- expect: inverted box inside `big`, by value and `inside_trans`.
- why: corner comparison composes even though the first box has no
  member point; no hidden validity premise may gate the law.

### stdlib/geometry/box-inside-order-greatest-intersection

- spec: §5.1.
- promise class: durable invariant.
- given: `small` inside each of `mid` and `wide`.
- expect: `small` inside their intersection; the closed intersection
  has exactly the stated corners.
- why: both premises contribute coordinate bounds to the same
  intersection. A one-sided law alone does not imply this direction.

### stdlib/geometry/box-inside-order-least-hull-inverted

- spec: §5.1.
- promise class: durable invariant.
- given: `small` and inverted both inside `mid`.
- expect: their hull inside `mid` by `hull_least`.
- why: the stored corners of the inverted operand still participate;
  the law does not discard it as an empty point set.

## Isolated refused reverse inclusions

For each refusal, first `ken check` the full accepted client above.
Start from a **fresh copy** of that accepted source and append **only
one** of the following fences, keeping the fixtures and all accepted
controls. Expect rejection at the appended theorem, not an import,
missing §5 export, malformed fixture or earlier declaration. The
three refused directions are not consequences of the three public
laws; a law silently stated without its premise would admit one.

### stdlib/geometry/reject-hull-inside-left-operand

- spec: §5.1.
- promise class: durable invariant.
- given: the same `small`, `mid` and `hull small mid` that were accepted
  as inside `mid`; `inside mid small` is independently `False`.
- expect: reject the claim that the hull is inside `small`.
- why: it includes `mid`'s wider corners, so it cannot fit in `small`.

```ken
theorem reject_hull_inside_left_operand
    : IsTrue (inside (hull small mid) small) =
  Proved
```

### stdlib/geometry/reject-left-inside-intersection

- spec: §5.1.
- promise class: durable invariant.
- given: the same `mid`, `small` and intersection accepted inside
  `small`; `inside mid small` is independently `False`.
- expect: reject the claim that `mid` fits in `intersect mid small`.
- why: the intersection narrows to `small`'s corners, while `mid`
  cannot fit inside `small`.

```ken
theorem reject_left_inside_intersection
    : IsTrue (inside mid (intersect mid small)) =
  Proved
```

### stdlib/geometry/reject-reverse-inverted-chain

- spec: §5.1.
- promise class: durable invariant.
- given: the accepted `inverted_chains_to_big` and direct
  `inverted_inside_big_value` on the same two boxes.
- expect: reject `inside big inverted`.
- why: the inverted box's stored corners do not enclose `big`.

```ken
theorem reject_reverse_inverted_chain
    : IsTrue (inside big inverted) =
  Proved
```

## Evidence boundary

The source Ken fences were extracted into temporary fenced clients
under `catalog/packages/Algorithm/Geometry/` in an isolated worktree
at `4f337ec26eaebf62d4951878e96809619aedc25f`, with only
Architect's ruled §5 package patch overlaid. The Ken CLI built in
Architect's same-base D0 scratch checked the full positive client at
rc 0. Each fresh positive-plus-one-refusal assembly returned rc 1,
`K0004 KernelRejected(TypeMismatch)`, at offset 1885 within the
appended theorem (each starts at 1884), not a missing import, fixture
or earlier proof. Logs are `/tmp/spec-author-box-inside-order-Positive.log`
and `/tmp/spec-author-box-inside-order-Refusal{1..3}.log`. The three
refusals distinguish reversed order bounds but do not prove the
general laws. This is **scratch-overlay** checking, not execution
against shipped §5 exports or the official conformance runner, CI,
publication, landing or a landed trusted-base measurement.
