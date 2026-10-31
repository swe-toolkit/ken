# Corner-wise inclusion yields structural box equality

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §12.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INSIDE-ANTISYMMETRY.md`.

This seed checks the public antisymmetry law on generic and closed
clients. Its finite boxes do not establish the generic theorem. The
cases are authored expectations; the seed has not been Ken-checked or
run by the official conformance runner. No CI, publication, landing,
or landed-trust result is claimed.

Provenance: expected behavior comes from §12, AC-2, and closed
natural-coordinate comparisons. I inspected the candidate-local package
test to cross-check public syntax and fixtures; it was not run and is
not conformance evidence. No external or `local/refs/` implementation
was consulted. No copyleft material informed this seed, so no
originality scan was run.

## Shared fixtures and accepted client

`small` and `same_corners` are both `(1,1)–(2,2)`. `inner` is inside
`large=(0,0)–(3,3)`, but the reverse comparison is false. `empty_a`
and `empty_b` are distinct boxes with inverted corners.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.Geometry.NatBox
  (Box, MkPoint, MkBox, inside, inside_antisym)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const same_corners : Box = MkBox (MkPoint one one) (MkPoint two two)
const inner : Box = MkBox (MkPoint one one) (MkPoint two two)
const large : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const empty_a : Box = MkBox (MkPoint three three) (MkPoint one one)
const empty_b : Box = MkBox (MkPoint two two) (MkPoint Zero Zero)

theorem generic_antisymmetry_application
    (a : Box) (b : Box)
    (ab : IsTrue (inside a b))
    (ba : IsTrue (inside b a))
  : Equal Box a b = inside_antisym a b ab ba

theorem small_inside_same_corners :
  IsTrue (inside small same_corners) = Proved

theorem same_corners_inside_small :
  IsTrue (inside same_corners small) = Proved

theorem small_equal_by_antisymmetry :
  Equal Box small same_corners =
  inside_antisym small same_corners
    small_inside_same_corners same_corners_inside_small
```

### stdlib/geometry/box-inside-antisymmetry-generic-and-closed

- spec: `spec/50-stdlib/67-geometry.md` §12.
- promise class: durable invariant.
- given: arbitrary `a` and `b` with both directional `IsTrue`
  premises; also the closed box `(1,1)–(2,2)` and an independently
  constructed box with the same corners.
- expect: the generic client applies `inside_antisym` to both premises;
  the two closed directional comparisons are accepted and the law
  yields `Equal Box` for the equal stored corners.
- measured: the public generic function shape and one closed
  application with both premises.
- claimed: mutual corner-wise inclusion implies structural equality.
- gap: one closed pair does not establish universal antisymmetry.
- why: the generic client retains both independent premise positions;
  the closed client proves them separately before applying the law.

## Refuse the missing directional premise

Append each block below alone to the shared accepted client. The direct
value controls are accepted before each refusal target.

### stdlib/geometry/reject-missing-reverse-inclusion

- spec: `spec/50-stdlib/67-geometry.md` §12.
- promise class: durable invariant.
- given: `inner=(1,1)–(2,2)` and `large=(0,0)–(3,3)`.
- expect: `inner` is inside `large`; the reverse is `False`; claiming
  `IsTrue` for that reverse direction is refused.
- measured: the target is the false second premise needed for
  `inside_antisym inner large`.
- claimed: the first inclusion alone does not supply mutual inclusion.
- gap: this closed pair does not establish the generic law.
- why: the accepted first-direction value and the `False` reverse
  control isolate the omitted premise, not a malformed application.

```ken
-- Accepted controls: only the inner-to-large direction holds.
theorem inner_inside_large_value :
  Equal Bool (inside inner large) True = Proved

theorem large_inside_inner_value :
  Equal Bool (inside large inner) False = Proved

-- Refuse: this missing reverse premise is not proved.
theorem reject_missing_reverse_inclusion :
  IsTrue (inside large inner) = Proved
```

### stdlib/geometry/reject-one-premise-antisymmetry

- spec: `spec/50-stdlib/67-geometry.md` §12.
- promise class: durable invariant.
- given: the same accepted inner-to-large proof and false reverse
  comparison above.
- expect: applying `inside_antisym` with only the first proof does
  not establish `Equal Box inner large`.
- measured: the one-argument application remains a function awaiting
  the reverse-premise proof, not a box equality.
- claimed: both directional premises are required by the public law.
- gap: this test does not enumerate other invalid arguments.
- why: the accepted generic application uses both premises, while this
  controlled attempt supplies only the true first direction.

```ken
-- Accepted controls: the forward premise is true; the reverse is false.
theorem inner_inside_large_for_one_premise :
  IsTrue (inside inner large) = Proved

theorem reverse_direction_for_one_premise_is_false :
  Equal Bool (inside large inner) False = Proved

-- Refuse: the result still requires the reverse-direction proof.
theorem reject_one_premise_antisymmetry :
  Equal Box inner large =
  inside_antisym inner large inner_inside_large_for_one_premise
```

## Refuse equality of distinct inverted boxes

### stdlib/geometry/reject-equality-of-distinct-inverted-boxes

- spec: `spec/50-stdlib/67-geometry.md` §§1.1, 12.
- promise class: durable invariant.
- given: `empty_a=(3,3)–(1,1)` and `empty_b=(2,2)–(0,0)`.
- expect: neither box is inside the other; each same-box structural
  equality is accepted; equality between their distinct stored corners
  is refused.
- measured: both directional `inside` values and reflexive
  `Equal Box` controls precede the false structural equality target.
- claimed: the law does not identify differently represented empty
  boxes.
- gap: two inverted examples do not characterize all empty boxes.
- why: the same-form `Refl` controls type-check; the final `Refl`
  fails because the stored corners differ, not because of proof form.

```ken
-- Accepted controls: neither inverted box includes the other.
theorem empty_a_inside_empty_b_value :
  Equal Bool (inside empty_a empty_b) False = Proved

theorem empty_b_inside_empty_a_value :
  Equal Bool (inside empty_b empty_a) False = Proved

-- Same-form reflexive controls for the structural equality target.
theorem empty_a_reflexive_equal : Equal Box empty_a empty_a = Refl
theorem empty_b_reflexive_equal : Equal Box empty_b empty_b = Refl

-- Refuse: distinct stored corners are not structurally equal.
theorem reject_distinct_inverted_box_equality :
  Equal Box empty_a empty_b = Refl
```
