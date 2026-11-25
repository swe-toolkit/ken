# Positive shared scaling preserves orientation

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §22,
`SPEC-GEOMETRY-ORIENT-SCALE`.
Architect AC-0: `evt_2zvkaccjkdjff`.

Architect reports AC-0 scratch checks at base
`80c900f1032cd608fc061eb8fea3ad4d729e1a2a`: `orient_scale` and
`compare::mul_cancel_r` checked without a new `Axiom`, and
`trusted_base()` remained unchanged as an identity set. The scratch
package target passed 10/10; its trust measurements were Order
`114 → 114` and NatOrientation `114 → 114`. These are
Architect-reported scratch results, not AC-T on an assembled candidate.

The fixture uses `(0,0), (1,0), (0,1)`, whose orientation is `Gt`.
Scaling every coordinate by `2` gives orientation `Gt`; scaling by
`0` collapses all points to the origin and gives `Eq`. The four
accepted rows below separate the unit control, law application, closed
equation, and zero-factor value. The two refusals each use a fresh
accepted prefix. The first only tests the law-form boundary; the second
tests the false equation at factor zero. The closed cases do not
establish the generic proof body.

## Accepted positive-scale client

The whole block accepts. `open_scale` and
`open_compare_mul_cancel` are selective-import clients of the two
public laws. They pin availability and type use, not proof-body
correctness. The four accepted rows are `unit_turn_gt`,
`doubled_turn_by_law`, `doubled_equation_closed`, and
`zero_scaled_is_eq`.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_scale)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Eq, Gt)
import Core.Logic.Transport (trans)
import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order (compare)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_scale
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (k : Nat)
  : Equal OrdResult
      (orient (MkPoint (mul xa (Suc k)) (mul ya (Suc k)))
              (MkPoint (mul xb (Suc k)) (mul yb (Suc k)))
              (MkPoint (mul xc (Suc k)) (mul yc (Suc k))))
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_scale xa ya xb yb xc yc k

theorem open_compare_mul_cancel (a : Nat) (b : Nat) (k : Nat)
  : Equal OrdResult (compare (mul a (Suc k)) (mul b (Suc k))) (compare a b) =
  (proof mul_cancel_r for compare) a b k

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem doubled_turn_by_law
  : Equal OrdResult
      (orient (MkPoint (mul n0 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n1 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n0 (Suc n1)) (mul n1 (Suc n1))))
      Gt =
  trans
    OrdResult
    (orient (MkPoint (mul n0 (Suc n1)) (mul n0 (Suc n1)))
            (MkPoint (mul n1 (Suc n1)) (mul n0 (Suc n1)))
            (MkPoint (mul n0 (Suc n1)) (mul n1 (Suc n1))))
    (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1))
    Gt
    (orient_scale n0 n0 n1 n0 n0 n1 n1)
    unit_turn_gt

theorem doubled_equation_closed
  : Equal OrdResult
      (orient (MkPoint (mul n0 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n1 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n0 (Suc n1)) (mul n1 (Suc n1))))
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) =
  Proved

theorem zero_scaled_is_eq
  : Equal OrdResult
      (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
              (MkPoint (mul n1 Zero) (mul n0 Zero))
              (MkPoint (mul n0 Zero) (mul n1 Zero)))
      Eq =
  Proved
```

### `stdlib/geometry/orient-scale-unit-turn`

- spec: §22.
- promise class: durable invariant.
- given: the unit triangle `(0,0), (1,0), (0,1)`.
- expect: `unit_turn_gt` accepts with result `Gt`.
- measured: the closed orientation of the original triangle.
- claimed: this triangle has positive orientation.
- gap: one closed value does not establish the generic scaling law.
- why: it is the accepted control for the later law application.

### `stdlib/geometry/orient-scale-doubled-by-law`

- spec: §22.
- promise class: durable invariant.
- given: the unit triangle scaled by `2 = Suc 1`.
- expect: `doubled_turn_by_law` accepts with result `Gt`.
- measured: the public `orient_scale` application and the accepted
  `unit_turn_gt` control.
- claimed: the public law preserves this triangle's orientation at a
  positive factor.
- gap: this closed application does not establish the generic proof.
- why: the law application agrees with the direct orientation result.

### `stdlib/geometry/orient-scale-doubled-equation`

- spec: §22.
- promise class: durable invariant.
- given: the law's equation for the unit triangle at factor `2`.
- expect: `doubled_equation_closed` accepts by `Proved`.
- measured: both closed orientation results at the positive factor.
- claimed: the equation holds for this positive-factor instance.
- gap: this finite instance does not establish the generic theorem.
- why: it independently checks the closed equation without applying
  `orient_scale`.

### `stdlib/geometry/orient-scale-zero-value`

- spec: §22.
- promise class: durable invariant.
- given: the unit triangle with every coordinate multiplied by `Zero`.
- expect: `zero_scaled_is_eq` accepts with result `Eq`.
- measured: all three points collapse to the origin.
- claimed: zero scaling does not preserve the original `Gt` result.
- gap: this value control alone does not establish the law's type or
  generic proof.
- why: it exposes the result used by the zero-factor refusal below.

## Refuse the zero-factor law-form twin

This block starts with a fresh accepted prefix containing
`open_scale`, `open_compare_mul_cancel`, and `unit_turn_gt`. Only the
factor in the target is changed to `Zero`; the proof still applies
`orient_scale` at factor `2`. This refusal checks the law-form boundary.
By itself, it does not establish that the law's equation is false at
factor zero.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_scale)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Eq, Gt)
import Core.Logic.Transport (trans)
import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order (compare)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_scale
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (k : Nat)
  : Equal OrdResult
      (orient (MkPoint (mul xa (Suc k)) (mul ya (Suc k)))
              (MkPoint (mul xb (Suc k)) (mul yb (Suc k)))
              (MkPoint (mul xc (Suc k)) (mul yc (Suc k))))
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_scale xa ya xb yb xc yc k

theorem open_compare_mul_cancel (a : Nat) (b : Nat) (k : Nat)
  : Equal OrdResult (compare (mul a (Suc k)) (mul b (Suc k))) (compare a b) =
  (proof mul_cancel_r for compare) a b k

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem zero_turn_by_law
  : Equal OrdResult
      (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
              (MkPoint (mul n1 Zero) (mul n0 Zero))
              (MkPoint (mul n0 Zero) (mul n1 Zero)))
      Gt =
  trans
    OrdResult
    (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
            (MkPoint (mul n1 Zero) (mul n0 Zero))
            (MkPoint (mul n0 Zero) (mul n1 Zero)))
    (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1))
    Gt
    (orient_scale n0 n0 n1 n0 n0 n1 n1)
    unit_turn_gt
```

### `stdlib/geometry/orient-scale-zero-law-form-refusal`

- spec: §22.
- promise class: durable invariant.
- given: a fresh accepted prefix, followed by `zero_turn_by_law`.
- expect: the prefix accepts; `zero_turn_by_law` is rejected with
  `KernelRejected TypeMismatch` in its declaration. Do not pin a span
  or GlobalIds.
- measured: only the factor in the claimed result changes from `2` to
  `Zero`; the proof still uses the factor-`2` law application.
- claimed: the public law's factor has the required form `Suc k`.
- gap: this refusal alone does not test whether the equation at zero is
  false; it checks that this proof term does not match the changed goal.
- controls: `doubled_turn_by_law` accepts at factor `2`; the accepted
  block separately proves the zero-scaled value is `Eq`.
- why: the factor-`2` application cannot prove the zero-factor claim.

## Refuse the equation at zero factor

This block also starts from a fresh accepted prefix containing
`open_scale`, `open_compare_mul_cancel`, and `unit_turn_gt`. It adds only
the equation at factor `Zero`, proved by `Proved`.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_scale)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Eq, Gt)
import Core.Logic.Transport (trans)
import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order (compare)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_scale
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (k : Nat)
  : Equal OrdResult
      (orient (MkPoint (mul xa (Suc k)) (mul ya (Suc k)))
              (MkPoint (mul xb (Suc k)) (mul yb (Suc k)))
              (MkPoint (mul xc (Suc k)) (mul yc (Suc k))))
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_scale xa ya xb yb xc yc k

theorem open_compare_mul_cancel (a : Nat) (b : Nat) (k : Nat)
  : Equal OrdResult (compare (mul a (Suc k)) (mul b (Suc k))) (compare a b) =
  (proof mul_cancel_r for compare) a b k

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem zero_equation_claim
  : Equal OrdResult
      (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
              (MkPoint (mul n1 Zero) (mul n0 Zero))
              (MkPoint (mul n0 Zero) (mul n1 Zero)))
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) =
  Proved
```

### `stdlib/geometry/orient-scale-zero-equation-refusal`

- spec: §22.
- promise class: durable invariant.
- given: a fresh accepted prefix, followed by `zero_equation_claim`.
- expect: the prefix accepts; `zero_equation_claim` is rejected with
  `KernelRejected TypeMismatch` in its declaration. Do not pin a span
  or GlobalIds.
- measured: the zero-scaled orientation is `Eq`; the original
  orientation is `Gt`.
- claimed: the law's equation is false at factor `Zero`.
- gap: this closed counterexample does not establish the generic law or
  proof body.
- controls: `unit_turn_gt` accepts in the prefix, and
  `zero_scaled_is_eq` accepts in the preceding client.
- why: `Proved` cannot inhabit the equality between `Eq` and `Gt`.

## Evidence boundary

The accepted and refused clients are transcribed from Architect AC-0's
measured scratch clients. AC-0 reports `ken check` success on the scratch
packages, no added `Axiom`, and unchanged `trusted_base()` as an identity
set; the existing `cat_geometry_nat_orientation` target passed 10/10
there. Its trust measurements were Order `114 → 114` and NatOrientation
`114 → 114`. These are Architect-reported scratch results, not AC-T on
an assembled candidate.

At exact assembled candidate
`4b666a3011e98fac972ccacd6e282b3efe7d82cb`, Foundation QA reports the
AC-T package target `cat_geometry_nat_orientation` passed 14/14. The
Order trusted `GlobalId` set was unchanged (`114 → 114`), as was the
NatOrientation set (`114 → 114`). Both zero-factor claims were rejected
at their own theorem as `KernelRejected(TypeMismatch)`. These are
candidate-local package results, not official Markdown-seed conformance.

This seed has not been run by the official conformance runner. No
official conformance, full CI, publication, landing, or landed-trust
claim is made.
