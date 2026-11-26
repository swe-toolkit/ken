# Reflecting across the diagonal flips the turn

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §23.
Frame: `docs/program/issues/SPEC-GEOMETRY-ORIENT-REFLECT-DIAGONAL.md`.
Architect AC-0: `evt_5mmyq8n4hjj3e`.

This seed is authored but unexecuted. No claim is made that the official
conformance runner has executed it.

The control triangle `(0,0), (1,0), (0,1)` has orientation `Gt`. Its
reflection across the diagonal exchanges each point's coordinates and has
orientation `Lt`. The open discriminator checks that a `Gt` result for an
arbitrary triangle is reflected to `Lt`; unlike the closed values, this
row cannot be decided by computation alone. The accepted and refused
closed claims do not establish the generic law.

## Accepted client

Every row below accepts. The selective imports expose the law and its
`ord_result_flip` result. `open_reflect` checks the public law at arbitrary
coordinates; the closed controls show the original and reflected turns;
`open_gt_reflects_to_lt` is the open discriminator.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_reflect_diagonal)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Gt, ord_result_flip)
import Core.Logic.Transport (cong, trans)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_reflect
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
  : Equal OrdResult
      (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
      (ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))) =
  orient_reflect_diagonal xa ya xb yb xc yc

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem reflected_turn_by_law
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n0 n1) (MkPoint n1 n0)) Lt =
  trans
    OrdResult
    (orient (MkPoint n0 n0) (MkPoint n0 n1) (MkPoint n1 n0))
    (ord_result_flip (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)))
    Lt
    (orient_reflect_diagonal n0 n0 n1 n0 n0 n1)
    (cong
      OrdResult
      OrdResult
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1))
      Gt
      ord_result_flip
      unit_turn_gt)

theorem reflected_turn_closed
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n0 n1) (MkPoint n1 n0)) Lt = Proved

theorem open_gt_reflects_to_lt
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
    (h : Equal OrdResult (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) Gt)
  : Equal OrdResult (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc)) Lt =
  trans
    OrdResult
    (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
    (ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)))
    Lt
    (orient_reflect_diagonal xa ya xb yb xc yc)
    (cong
      OrdResult
      OrdResult
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))
      Gt
      ord_result_flip
      h)
```

### stdlib/geometry/orient-reflect-diagonal-control

- spec: §23; reflection exchanges each point's coordinates and flips the
  orientation result.
- promise class: durable invariant.
- given: the control triangle `(0,0), (1,0), (0,1)` and its diagonal
  reflection `(0,0), (0,1), (1,0)`.
- expect: the original turn is `Gt`; the reflected turn is `Lt`; the
  accepted law application and open discriminator accept.
- measured: the original and reflected closed values, the public law
  instance, and the open result transformation from `Gt` to `Lt`.
- claimed: diagonal reflection flips this control triangle's turn and
  applies the same flip to an open `Gt` premise.
- gap: the closed values do not establish the generic theorem; the open
  row uses the theorem and does not establish its proof body.
- controls: `unit_turn_gt` fixes the original value; the independent
  `reflected_turn_closed` fixes the reflected value. `open_gt_reflects_to_lt`
  rules out a law that preserves the turn.
- why: swapping both coordinates exchanges the two cross sums, so their
  comparison reverses.

## Refuse the law-form preservation twin

Use a fresh copy of the accepted prefix containing `open_reflect` and
`unit_turn_gt`, then append only the theorem below. It differs from
`reflected_turn_by_law` only in claiming `Gt` rather than `Lt`; the
`trans` endpoint changes to match that claimed result. The expected
refusal is `KernelRejected(TypeMismatch)` in this theorem. Do not pin a
span or GlobalIds.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_reflect_diagonal)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Gt, ord_result_flip)
import Core.Logic.Transport (cong, trans)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_reflect
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
  : Equal OrdResult
      (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
      (ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))) =
  orient_reflect_diagonal xa ya xb yb xc yc

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem reflected_turn_preserved_by_law
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n0 n1) (MkPoint n1 n0)) Gt =
  trans
    OrdResult
    (orient (MkPoint n0 n0) (MkPoint n0 n1) (MkPoint n1 n0))
    (ord_result_flip (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)))
    Gt
    (orient_reflect_diagonal n0 n0 n1 n0 n0 n1)
    (cong
      OrdResult
      OrdResult
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1))
      Gt
      ord_result_flip
      unit_turn_gt)
```

### stdlib/geometry/reject-orient-reflect-preserved-by-law

- spec: §23; reflection concludes with `ord_result_flip`, not the
  original orientation.
- promise class: durable invariant.
- given: the accepted prefix and the same reflected control triangle as
  `orient-reflect-diagonal-control`.
- expect: the prefix accepts; `reflected_turn_preserved_by_law` refuses
  with `KernelRejected(TypeMismatch)`.
- measured: only the claimed result changes from `Lt` to `Gt` in the
  law-form control.
- claimed: the law cannot preserve the original turn under reflection.
- gap: this closed law application does not distinguish every possible
  incorrect law or proof.
- controls: `reflected_turn_by_law` accepts with `Lt`; the false twin
  uses the same reflection, law, premise and proof form.
- why: the reflected value is `Lt`, so the law cannot prove `Gt`.

## Refuse the open wrong-Gt discriminator

Use another fresh copy of the accepted prefix containing `open_reflect`
and `unit_turn_gt`, then append only the theorem below. It differs from
`open_gt_reflects_to_lt` only in claiming `Gt` rather than `Lt`; the
`trans` endpoint changes to match that claimed result. The expected
refusal is `KernelRejected(TypeMismatch)` in this theorem. This open pair
is the discriminator: a preservation law without the flip would accept
this twin and refuse its accepted partner. Do not pin a span or
GlobalIds.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_reflect_diagonal)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Gt, ord_result_flip)
import Core.Logic.Transport (cong, trans)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_reflect
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
  : Equal OrdResult
      (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
      (ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))) =
  orient_reflect_diagonal xa ya xb yb xc yc

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem open_gt_reflects_to_gt
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
    (h : Equal OrdResult (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) Gt)
  : Equal OrdResult (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc)) Gt =
  trans
    OrdResult
    (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
    (ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)))
    Gt
    (orient_reflect_diagonal xa ya xb yb xc yc)
    (cong
      OrdResult
      OrdResult
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))
      Gt
      ord_result_flip
      h)
```

### stdlib/geometry/reject-open-orient-reflect-to-wrong-gt

- spec: §23; open result transformation under `orient_reflect_diagonal`.
- promise class: durable invariant.
- given: an open triangle with a proof its orientation is `Gt`, and its
  coordinate-wise reflection.
- expect: the accepted prefix succeeds; `open_gt_reflects_to_gt` refuses
  with `KernelRejected(TypeMismatch)`.
- measured: the open accepted partner derives `Lt` from the law; the twin
  differs only in claiming `Gt`.
- claimed: an open `Gt` premise cannot be transported to `Gt` after
  reflecting each point.
- gap: this pair tests the law's flip through an open premise, not all
  possible orientations or coordinates.
- controls: `open_gt_reflects_to_lt` accepts with the same input,
  premise and proof construction, differing only in its result claim.
- why: the result flip maps `Gt` to `Lt`.

## Refuse the closed preserved-value twin

Use a third fresh copy of the accepted prefix containing `open_reflect`
and `unit_turn_gt`, then append only the false closed claim below. It
differs from `reflected_turn_closed` only in claiming `Gt` rather than
`Lt`. The expected refusal is `KernelRejected(TypeMismatch)` in this
theorem. Do not pin a span or GlobalIds.

```ken
import Algorithm.Geometry.NatOrientation (orient, orient_reflect_diagonal)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Gt, ord_result_flip)
import Core.Logic.Transport (cong, trans)

const n0 : Nat = Zero
const n1 : Nat = Suc n0

theorem open_reflect
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
  : Equal OrdResult
      (orient (MkPoint ya xa) (MkPoint yb xb) (MkPoint yc xc))
      (ord_result_flip (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))) =
  orient_reflect_diagonal xa ya xb yb xc yc

theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem reflected_turn_preserved_closed
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n0 n1) (MkPoint n1 n0)) Gt = Proved
```

### stdlib/geometry/reject-orient-reflect-preserved-closed

- spec: §23; the reflected control triangle has turn `Lt`.
- promise class: durable invariant.
- given: the reflected control triangle `(0,0), (0,1), (1,0)`.
- expect: the accepted prefix succeeds; `reflected_turn_preserved_closed`
  refuses with `KernelRejected(TypeMismatch)`.
- measured: `reflected_turn_closed` accepts with `Lt`; the false twin
  claims `Gt` for the same closed expression.
- claimed: the closed reflected turn is not the original `Gt`.
- gap: the closed equation does not establish the generic reflection law.
- controls: the accepted closed result and false twin use the same
  expression and differ only in the claimed result.
- why: the reflection reverses the control triangle's orientation.

## Evidence boundary

These are authored expectations grounded in Architect AC-0. The open
wrong-Gt pair is necessary because closed outcomes can be computed without
the public law. No official conformance-runner execution, CI, publication,
landing, or landed-trust measurement is claimed.
