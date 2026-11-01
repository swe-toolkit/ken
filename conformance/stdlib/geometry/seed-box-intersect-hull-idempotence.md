# Intersection and hull are structurally idempotent

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §13.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INTERSECT-HULL-IDEMPOTENCE.md`.

Intersection and hull of a box with itself restore its stored corners,
including for inverted boxes. The closed controls do not substitute for
the two generic checked proofs in `Algorithm.Geometry.NatBox`.

## Accepted client

`large=(0,0)–(4,4)` and `small=(1,1)–(2,2)` are different valid
boxes; `small` is strictly inside `large`. `inverted=(3,3)–(1,1)`
retains its own corners even though it has no members. The accepted
client applies both public laws to `large` and `inverted` and checks
their closed results independently.

```ken
import Algorithm.Geometry.NatBox
  (Box, MkBox, MkPoint, intersect, hull, intersect_idempotent, hull_idempotent)

const zero : Nat = Zero

const one : Nat = Suc zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const large : Box = MkBox (MkPoint zero zero) (MkPoint four four)

const small : Box = MkBox (MkPoint one one) (MkPoint two two)

const inverted : Box = MkBox (MkPoint three three) (MkPoint one one)

theorem large_intersection_value : Equal Box (intersect large large) large = Refl

theorem large_hull_value : Equal Box (hull large large) large = Refl

theorem large_intersection_law : Equal Box (intersect large large) large =
  intersect_idempotent large

theorem large_hull_law : Equal Box (hull large large) large =
  hull_idempotent large

theorem inverted_intersection_value : Equal Box (intersect inverted inverted) inverted = Refl

theorem inverted_hull_value : Equal Box (hull inverted inverted) inverted = Refl

theorem inverted_intersection_law : Equal Box (intersect inverted inverted) inverted =
  intersect_idempotent inverted

theorem inverted_hull_law : Equal Box (hull inverted inverted) inverted =
  hull_idempotent inverted

theorem small_large_intersection_value : Equal Box (intersect small large) small = Refl

theorem same_form_true_control : Equal Box (intersect large large) large = Refl
```

### `stdlib/geometry/box-intersection-hull-idempotence`

- spec: §13.
- promise class: durable invariant.
- given: `large` and the inverted `(3,3)–(1,1)` box.
- expect: both operations with two identical operands return the original
  stored box; both public theorems apply on both inputs.
- measured: when run, the complete accepted program checks the four
  law applications and independent `Refl` value controls.
- claimed: the two public laws restore structural equality for these
  regular and inverted boxes.
- gap: finite instances cannot establish the generic laws; their
  Transparent proof bodies must also check on open `Box` input.

## Isolated refusal

Start from a fresh complete copy of the accepted client and append only
the following theorem. The accepted `same_form_true_control` keeps
`Equal Box`, `Refl`, and the right-hand `large` fixed; the false claim
changes the intersection's left operand to `small`. Because `small` is
strictly inside `large`, the intersection is `small`, not `large`.

### `stdlib/geometry/reject-small-intersection-as-large`

- spec: §13.
- promise class: durable invariant.
- given: distinct, nested `small` and `large`.
- expect: the false claim rejects at its own `Refl`, after the accepted
  same-form equality and actual small-intersection result check.
- measured: when run, the accepted prefix checks; the isolated false
  equality is rejected at its own proof term.
- claimed: idempotence does not assert that intersecting different
  nested boxes returns the larger box.
- gap: the refusal does not establish either generic theorem.

```ken
theorem seed_neg : Equal Box (intersect small large) large = Refl
```

## Evidence boundary

This authored seed requires root-loading `Algorithm.Geometry.NatBox`.
Standalone `ken check` of a non-catalog seed cannot resolve the catalog
import on this base. The package-owned test checks its complete accepted
client and isolated refusal locally; that is not official conformance-
runner execution, CI, publication, landing or a landed trust result.
