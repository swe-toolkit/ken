# Natural-point orientation

Three natural points determine an exact turn by comparing two cross sums.
The eight public laws include `orient_shared_x` and `orient_shared_y`: each
returns `Eq` when all three points share the named coordinate. They do not
state a general collinearity theorem for arbitrary distinct points.
`orient_translate` preserves the turn when the same natural offset is added
to all three points on each axis. It does not apply to moving just one point.
`orient_scale` preserves the turn when all six coordinates are multiplied by
one positive factor.

## Definition

Both shared-axis proofs turn one cross sum into the other with the private
`orient_reorder`, a swap of the inner sum, and commutativity of each product.
The checked `compare::self_eq` law closes the resulting turn.

The translation proof inducts on the horizontal offset and then on the
vertical offset. Moving every point one unit horizontally adds the sum of
the three y coordinates to each cross sum. Moving every point one unit
vertically adds the sum of the three x coordinates to each cross sum.
Private helpers use distributivity and rearrange the terms to expose the
common summand; [`compare::add_cancel_r`](../../Data/Numeric/Nat/Order.ken.md)
then cancels it from the comparison. No signed cross product is needed.

Scaling every coordinate by `Suc k` contributes two copies of that factor
to each product in the cross sums. Associativity and commutativity group
those factors, and distributivity moves them outside each sum.
[`compare::mul_cancel_r`](../../Data/Numeric/Nat/Order.ken.md) then cancels
the positive right factor twice. A zero factor instead collapses the three
points to the origin, giving `Eq` even when the original points turn.

```ken
import Algorithm.Geometry.NatBox (Point, MkPoint)

import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_result_flip)

import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l)

import Data.Numeric.Nat.Order (compare)

pub fn orient (a : Point) (b : Point) (c : Point) : OrdResult =
  match a {
    MkPoint xa ya ↦
      match b {
        MkPoint xb yb ↦
          match c {
            MkPoint xc yc ↦
              compare
                (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
          }
      }
  }

pub theorem orient_cross
      (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
    : Equal OrdResult
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))
        (compare
          (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
          (add (add (mul ya xb) (mul yb xc)) (mul yc xa))) =
  Refl

pub theorem orient_swap
      (a : Point) (b : Point) (c : Point)
    : Equal OrdResult (orient b a c) (ord_result_flip (orient a b c)) =
  match a {
    MkPoint xa ya ↦
      match b {
        MkPoint xb yb ↦
          match c {
            MkPoint xc yc ↦
              (trans
                OrdResult
                (compare
                  (add (add (mul xb ya) (mul xa yc)) (mul xc yb))
                  (add (add (mul yb xa) (mul ya xc)) (mul yc xb)))
                (compare
                  (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                  (add (add (mul xa yb) (mul xb yc)) (mul xc ya)))
                (ord_result_flip
                  (compare
                    (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))))
                (trans
                  OrdResult
                  (compare
                    (add (add (mul xb ya) (mul xa yc)) (mul xc yb))
                    (add (add (mul yb xa) (mul ya xc)) (mul yc xb)))
                  (compare
                    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                    (add (add (mul yb xa) (mul ya xc)) (mul yc xb)))
                  (compare
                    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                    (add (add (mul xa yb) (mul xb yc)) (mul xc ya)))
                  (cong
                    Nat
                    OrdResult
                    (add (add (mul xb ya) (mul xa yc)) (mul xc yb))
                    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                    (λu. (compare u (add (add (mul yb xa) (mul ya xc)) (mul yc xb))))
                    (trans
                      Nat
                      (add (add (mul xb ya) (mul xa yc)) (mul xc yb))
                      (add (add (mul xb ya) (mul xc yb)) (mul xa yc))
                      (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                      (orient_reorder (mul xb ya) (mul xa yc) (mul xc yb))
                      (trans
                        Nat
                        (add (add (mul xb ya) (mul xc yb)) (mul xa yc))
                        (add (add (mul ya xb) (mul xc yb)) (mul xa yc))
                        (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                        (cong
                          Nat
                          Nat
                          (mul xb ya)
                          (mul ya xb)
                          (λu. (add (add u (mul xc yb)) (mul xa yc)))
                          ((proof comm for mul) xb ya))
                        (trans
                          Nat
                          (add (add (mul ya xb) (mul xc yb)) (mul xa yc))
                          (add (add (mul ya xb) (mul yb xc)) (mul xa yc))
                          (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                          (cong
                            Nat
                            Nat
                            (mul xc yb)
                            (mul yb xc)
                            (λu. (add (add (mul ya xb) u) (mul xa yc)))
                            ((proof comm for mul) xc yb))
                          (cong
                            Nat
                            Nat
                            (mul xa yc)
                            (mul yc xa)
                            (λu. (add (add (mul ya xb) (mul yb xc)) u))
                            ((proof comm for mul) xa yc))))))
                  (cong
                    Nat
                    OrdResult
                    (add (add (mul yb xa) (mul ya xc)) (mul yc xb))
                    (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                    (compare (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
                    (trans
                      Nat
                      (add (add (mul yb xa) (mul ya xc)) (mul yc xb))
                      (add (add (mul yb xa) (mul yc xb)) (mul ya xc))
                      (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                      (orient_reorder (mul yb xa) (mul ya xc) (mul yc xb))
                      (trans
                        Nat
                        (add (add (mul yb xa) (mul yc xb)) (mul ya xc))
                        (add (add (mul xa yb) (mul yc xb)) (mul ya xc))
                        (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                        (cong
                          Nat
                          Nat
                          (mul yb xa)
                          (mul xa yb)
                          (λu. (add (add u (mul yc xb)) (mul ya xc)))
                          ((proof comm for mul) yb xa))
                        (trans
                          Nat
                          (add (add (mul xa yb) (mul yc xb)) (mul ya xc))
                          (add (add (mul xa yb) (mul xb yc)) (mul ya xc))
                          (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                          (cong
                            Nat
                            Nat
                            (mul yc xb)
                            (mul xb yc)
                            (λu. (add (add (mul xa yb) u) (mul ya xc)))
                            ((proof comm for mul) yc xb))
                          (cong
                            Nat
                            Nat
                            (mul ya xc)
                            (mul xc ya)
                            (λu. (add (add (mul xa yb) (mul xb yc)) u))
                            ((proof comm for mul) ya xc)))))))
                ((proof swap for compare)
                  (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                  (add (add (mul ya xb) (mul yb xc)) (mul yc xa))))
          }
      }
  }

pub theorem orient_rotation
      (a : Point) (b : Point) (c : Point)
    : Equal OrdResult (orient a b c) (orient b c a) =
  match a {
    MkPoint xa ya ↦
      match b {
        MkPoint xb yb ↦
          match c {
            MkPoint xc yc ↦
              (trans
                OrdResult
                (compare
                  (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                  (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
                (compare
                  (add (add (mul xb yc) (mul xc ya)) (mul xa yb))
                  (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
                (compare
                  (add (add (mul xb yc) (mul xc ya)) (mul xa yb))
                  (add (add (mul yb xc) (mul yc xa)) (mul ya xb)))
                (cong
                  Nat
                  OrdResult
                  (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                  (add (add (mul xb yc) (mul xc ya)) (mul xa yb))
                  (λu. (compare u (add (add (mul ya xb) (mul yb xc)) (mul yc xa))))
                  (trans
                    Nat
                    (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                    (add (mul xa yb) (add (mul xb yc) (mul xc ya)))
                    (add (add (mul xb yc) (mul xc ya)) (mul xa yb))
                    (sym
                      Nat
                      (add (mul xa yb) (add (mul xb yc) (mul xc ya)))
                      (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
                      ((proof assoc for add) (mul xa yb) (mul xb yc) (mul xc ya)))
                    ((proof comm for add) (mul xa yb) (add (mul xb yc) (mul xc ya)))))
                (cong
                  Nat
                  OrdResult
                  (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                  (add (add (mul yb xc) (mul yc xa)) (mul ya xb))
                  (compare (add (add (mul xb yc) (mul xc ya)) (mul xa yb)))
                  (trans
                    Nat
                    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                    (add (mul ya xb) (add (mul yb xc) (mul yc xa)))
                    (add (add (mul yb xc) (mul yc xa)) (mul ya xb))
                    (sym
                      Nat
                      (add (mul ya xb) (add (mul yb xc) (mul yc xa)))
                      (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
                      ((proof assoc for add) (mul ya xb) (mul yb xc) (mul yc xa)))
                    ((proof comm for add) (mul ya xb) (add (mul yb xc) (mul yc xa))))))
          }
      }
  }

pub theorem orient_repeated_point (a : Point) (c : Point) : Equal OrdResult (orient a a c) Eq =
  match a {
    MkPoint xa ya ↦
      match c {
        MkPoint xc yc ↦
          (trans
            OrdResult
            (compare
              (add (add (mul xa ya) (mul xa yc)) (mul xc ya))
              (add (add (mul ya xa) (mul ya xc)) (mul yc xa)))
            (compare
              (add (add (mul ya xa) (mul ya xc)) (mul yc xa))
              (add (add (mul ya xa) (mul ya xc)) (mul yc xa)))
            Eq
            (cong
              Nat
              OrdResult
              (add (add (mul xa ya) (mul xa yc)) (mul xc ya))
              (add (add (mul ya xa) (mul ya xc)) (mul yc xa))
              (λu. (compare u (add (add (mul ya xa) (mul ya xc)) (mul yc xa))))
              (trans
                Nat
                (add (add (mul xa ya) (mul xa yc)) (mul xc ya))
                (add (add (mul xa ya) (mul xc ya)) (mul xa yc))
                (add (add (mul ya xa) (mul ya xc)) (mul yc xa))
                (orient_reorder (mul xa ya) (mul xa yc) (mul xc ya))
                (trans
                  Nat
                  (add (add (mul xa ya) (mul xc ya)) (mul xa yc))
                  (add (add (mul ya xa) (mul xc ya)) (mul xa yc))
                  (add (add (mul ya xa) (mul ya xc)) (mul yc xa))
                  (cong
                    Nat
                    Nat
                    (mul xa ya)
                    (mul ya xa)
                    (λu. (add (add u (mul xc ya)) (mul xa yc)))
                    ((proof comm for mul) xa ya))
                  (trans
                    Nat
                    (add (add (mul ya xa) (mul xc ya)) (mul xa yc))
                    (add (add (mul ya xa) (mul ya xc)) (mul xa yc))
                    (add (add (mul ya xa) (mul ya xc)) (mul yc xa))
                    (cong
                      Nat
                      Nat
                      (mul xc ya)
                      (mul ya xc)
                      (λu. (add (add (mul ya xa) u) (mul xa yc)))
                      ((proof comm for mul) xc ya))
                    (cong
                      Nat
                      Nat
                      (mul xa yc)
                      (mul yc xa)
                      (λu. (add (add (mul ya xa) (mul ya xc)) u))
                      ((proof comm for mul) xa yc))))))
            ((proof self_eq for compare) (add (add (mul ya xa) (mul ya xc)) (mul yc xa))))
      }
  }

pub theorem orient_shared_x
      (x : Nat) (ya : Nat) (yb : Nat) (yc : Nat)
    : Equal OrdResult (orient (MkPoint x ya) (MkPoint x yb) (MkPoint x yc)) Eq =
  (trans
    OrdResult
    (compare
      (add (add (mul x yb) (mul x yc)) (mul x ya))
      (add (add (mul ya x) (mul yb x)) (mul yc x)))
    (compare
      (add (add (mul ya x) (mul yb x)) (mul yc x))
      (add (add (mul ya x) (mul yb x)) (mul yc x)))
    Eq
    (cong
      Nat
      OrdResult
      (add (add (mul x yb) (mul x yc)) (mul x ya))
      (add (add (mul ya x) (mul yb x)) (mul yc x))
      (λu. (compare u (add (add (mul ya x) (mul yb x)) (mul yc x))))
      (trans
        Nat
        (add (add (mul x yb) (mul x yc)) (mul x ya))
        (add (add (mul x yb) (mul x ya)) (mul x yc))
        (add (add (mul ya x) (mul yb x)) (mul yc x))
        (orient_reorder (mul x yb) (mul x yc) (mul x ya))
        (trans
          Nat
          (add (add (mul x yb) (mul x ya)) (mul x yc))
          (add (add (mul x ya) (mul x yb)) (mul x yc))
          (add (add (mul ya x) (mul yb x)) (mul yc x))
          (cong
            Nat
            Nat
            (add (mul x yb) (mul x ya))
            (add (mul x ya) (mul x yb))
            (λu. (add u (mul x yc)))
            ((proof comm for add) (mul x yb) (mul x ya)))
          (trans
            Nat
            (add (add (mul x ya) (mul x yb)) (mul x yc))
            (add (add (mul ya x) (mul x yb)) (mul x yc))
            (add (add (mul ya x) (mul yb x)) (mul yc x))
            (cong
              Nat
              Nat
              (mul x ya)
              (mul ya x)
              (λu. (add (add u (mul x yb)) (mul x yc)))
              ((proof comm for mul) x ya))
            (trans
              Nat
              (add (add (mul ya x) (mul x yb)) (mul x yc))
              (add (add (mul ya x) (mul yb x)) (mul x yc))
              (add (add (mul ya x) (mul yb x)) (mul yc x))
              (cong
                Nat
                Nat
                (mul x yb)
                (mul yb x)
                (λu. (add (add (mul ya x) u) (mul x yc)))
                ((proof comm for mul) x yb))
              (cong
                Nat
                Nat
                (mul x yc)
                (mul yc x)
                (λu. (add (add (mul ya x) (mul yb x)) u))
                ((proof comm for mul) x yc)))))))
    ((proof self_eq for compare) (add (add (mul ya x) (mul yb x)) (mul yc x))))

pub theorem orient_shared_y
      (y : Nat) (xa : Nat) (xb : Nat) (xc : Nat)
    : Equal OrdResult (orient (MkPoint xa y) (MkPoint xb y) (MkPoint xc y)) Eq =
  (trans
    OrdResult
    (compare
      (add (add (mul xa y) (mul xb y)) (mul xc y))
      (add (add (mul y xb) (mul y xc)) (mul y xa)))
    (compare
      (add (add (mul y xb) (mul y xc)) (mul y xa))
      (add (add (mul y xb) (mul y xc)) (mul y xa)))
    Eq
    (cong
      Nat
      OrdResult
      (add (add (mul xa y) (mul xb y)) (mul xc y))
      (add (add (mul y xb) (mul y xc)) (mul y xa))
      (λu. (compare u (add (add (mul y xb) (mul y xc)) (mul y xa))))
      (trans
        Nat
        (add (add (mul xa y) (mul xb y)) (mul xc y))
        (add (add (mul xb y) (mul xa y)) (mul xc y))
        (add (add (mul y xb) (mul y xc)) (mul y xa))
        (cong
          Nat
          Nat
          (add (mul xa y) (mul xb y))
          (add (mul xb y) (mul xa y))
          (λu. (add u (mul xc y)))
          ((proof comm for add) (mul xa y) (mul xb y)))
        (trans
          Nat
          (add (add (mul xb y) (mul xa y)) (mul xc y))
          (add (add (mul xb y) (mul xc y)) (mul xa y))
          (add (add (mul y xb) (mul y xc)) (mul y xa))
          (orient_reorder (mul xb y) (mul xa y) (mul xc y))
          (trans
            Nat
            (add (add (mul xb y) (mul xc y)) (mul xa y))
            (add (add (mul y xb) (mul xc y)) (mul xa y))
            (add (add (mul y xb) (mul y xc)) (mul y xa))
            (cong
              Nat
              Nat
              (mul xb y)
              (mul y xb)
              (λu. (add (add u (mul xc y)) (mul xa y)))
              ((proof comm for mul) xb y))
            (trans
              Nat
              (add (add (mul y xb) (mul xc y)) (mul xa y))
              (add (add (mul y xb) (mul y xc)) (mul xa y))
              (add (add (mul y xb) (mul y xc)) (mul y xa))
              (cong
                Nat
                Nat
                (mul xc y)
                (mul y xc)
                (λu. (add (add (mul y xb) u) (mul xa y)))
                ((proof comm for mul) xc y))
              (cong
                Nat
                Nat
                (mul xa y)
                (mul y xa)
                (λu. (add (add (mul y xb) (mul y xc)) u))
                ((proof comm for mul) xa y)))))))
    ((proof self_eq for compare) (add (add (mul y xb) (mul y xc)) (mul y xa))))

pub theorem orient_translate
      (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (dx : Nat) (dy : Nat)
    : Equal OrdResult
        (orient
          (MkPoint (add xa dx) (add ya dy))
          (MkPoint (add xb dx) (add yb dy))
          (MkPoint (add xc dx) (add yc dy)))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  match dx {
    Zero ↦ orient_translate_y xa ya xb yb xc yc dy;
    Suc k ↦
      trans
        OrdResult
        (orient
          (MkPoint (Suc (add xa k)) (add ya dy))
          (MkPoint (Suc (add xb k)) (add yb dy))
          (MkPoint (Suc (add xc k)) (add yc dy)))
        (orient
          (MkPoint (add xa k) (add ya dy))
          (MkPoint (add xb k) (add yb dy))
          (MkPoint (add xc k) (add yc dy)))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))
        (orient_step_x (add xa k) (add ya dy) (add xb k) (add yb dy) (add xc k) (add yc dy))
        (orient_translate xa ya xb yb xc yc k dy)
  }

theorem orient_translate_y
      (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (dy : Nat)
    : Equal OrdResult
        (orient (MkPoint xa (add ya dy)) (MkPoint xb (add yb dy)) (MkPoint xc (add yc dy)))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  match dy {
    Zero ↦ Refl;
    Suc k ↦
      trans
        OrdResult
        (orient
          (MkPoint xa (Suc (add ya k)))
          (MkPoint xb (Suc (add yb k)))
          (MkPoint xc (Suc (add yc k))))
        (orient (MkPoint xa (add ya k)) (MkPoint xb (add yb k)) (MkPoint xc (add yc k)))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc))
        (orient_step_y xa (add ya k) xb (add yb k) xc (add yc k))
        (orient_translate_y xa ya xb yb xc yc k)
  }

theorem orient_step_x
      (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
    : Equal OrdResult
        (orient (MkPoint (Suc xa) ya) (MkPoint (Suc xb) yb) (MkPoint (Suc xc) yc))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_common_addend
    (add (add (mul (Suc xa) yb) (mul (Suc xb) yc)) (mul (Suc xc) ya))
    (add (add (mul ya (Suc xb)) (mul yb (Suc xc))) (mul yc (Suc xa)))
    (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
    (add (add yb yc) ya)
    (trans
      Nat
      (add (add (mul (Suc xa) yb) (mul (Suc xb) yc)) (mul (Suc xc) ya))
      (add (add (add (mul xa yb) yb) (add (mul xb yc) yc)) (add (mul xc ya) ya))
      (add (add (add (mul xa yb) (mul xb yc)) (mul xc ya)) (add (add yb yc) ya))
      (orient_sum_congr
        (mul (Suc xa) yb)
        (add (mul xa yb) yb)
        (mul (Suc xb) yc)
        (add (mul xb yc) yc)
        (mul (Suc xc) ya)
        (add (mul xc ya) ya)
        (orient_shift_product xa yb)
        (orient_shift_product xb yc)
        (orient_shift_product xc ya))
      (orient_interleave (mul xa yb) yb (mul xb yc) yc (mul xc ya) ya))
    (trans
      Nat
      (add (add (mul ya (Suc xb)) (mul yb (Suc xc))) (mul yc (Suc xa)))
      (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (add (add ya yb) yc))
      (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (add (add yb yc) ya))
      (orient_interleave (mul ya xb) ya (mul yb xc) yb (mul yc xa) yc)
      (cong
        Nat
        Nat
        (add (add ya yb) yc)
        (add (add yb yc) ya)
        (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
        (sym Nat (add (add yb yc) ya) (add (add ya yb) yc) (orient_rotate_sum ya yb yc))))

theorem orient_step_y
      (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat)
    : Equal OrdResult
        (orient (MkPoint xa (Suc ya)) (MkPoint xb (Suc yb)) (MkPoint xc (Suc yc)))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_common_addend
    (add (add (mul xa (Suc yb)) (mul xb (Suc yc))) (mul xc (Suc ya)))
    (add (add (mul (Suc ya) xb) (mul (Suc yb) xc)) (mul (Suc yc) xa))
    (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
    (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
    (add (add xa xb) xc)
    (orient_interleave (mul xa yb) xa (mul xb yc) xb (mul xc ya) xc)
    (trans
      Nat
      (add (add (mul (Suc ya) xb) (mul (Suc yb) xc)) (mul (Suc yc) xa))
      (add (add (add (mul ya xb) xb) (add (mul yb xc) xc)) (add (mul yc xa) xa))
      (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (add (add xa xb) xc))
      (orient_sum_congr
        (mul (Suc ya) xb)
        (add (mul ya xb) xb)
        (mul (Suc yb) xc)
        (add (mul yb xc) xc)
        (mul (Suc yc) xa)
        (add (mul yc xa) xa)
        (orient_shift_product ya xb)
        (orient_shift_product yb xc)
        (orient_shift_product yc xa))
      (trans
        Nat
        (add (add (add (mul ya xb) xb) (add (mul yb xc) xc)) (add (mul yc xa) xa))
        (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (add (add xb xc) xa))
        (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (add (add xa xb) xc))
        (orient_interleave (mul ya xb) xb (mul yb xc) xc (mul yc xa) xa)
        (cong
          Nat
          Nat
          (add (add xb xc) xa)
          (add (add xa xb) xc)
          (add (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
          (orient_rotate_sum xa xb xc))))

theorem orient_common_addend
      (l2 : Nat)
      (r2 : Nat)
      (l : Nat)
      (r : Nat)
      (s : Nat)
      (left : Equal Nat l2 (add l s))
      (right : Equal Nat r2 (add r s))
    : Equal OrdResult (compare l2 r2) (compare l r) =
  trans
    OrdResult
    (compare l2 r2)
    (compare (add l s) r2)
    (compare l r)
    (cong Nat OrdResult l2 (add l s) (λu. (compare u r2)) left)
    (trans
      OrdResult
      (compare (add l s) r2)
      (compare (add l s) (add r s))
      (compare l r)
      (cong Nat OrdResult r2 (add r s) (compare (add l s)) right)
      ((proof add_cancel_r for compare) l r s))

theorem orient_shift_product (a : Nat) (b : Nat) : Equal Nat (mul (Suc a) b) (add (mul a b) b) =
  trans
    Nat
    (mul (Suc a) b)
    (add (mul a b) (mul (Suc Zero) b))
    (add (mul a b) b)
    (mul_add_distrib_l a (Suc Zero) b)
    (cong Nat Nat (mul (Suc Zero) b) b (add (mul a b)) ((proof one_l for mul) b))

theorem orient_sum_congr
      (a : Nat)
      (a2 : Nat)
      (b : Nat)
      (b2 : Nat)
      (c : Nat)
      (c2 : Nat)
      (p : Equal Nat a a2)
      (q : Equal Nat b b2)
      (r : Equal Nat c c2)
    : Equal Nat (add (add a b) c) (add (add a2 b2) c2) =
  trans
    Nat
    (add (add a b) c)
    (add (add a2 b) c)
    (add (add a2 b2) c2)
    (cong Nat Nat a a2 (λu. (add (add u b) c)) p)
    (trans
      Nat
      (add (add a2 b) c)
      (add (add a2 b2) c)
      (add (add a2 b2) c2)
      (cong Nat Nat b b2 (λu. (add (add a2 u) c)) q)
      (cong Nat Nat c c2 (add (add a2 b2)) r))

theorem orient_interleave
      (a : Nat) (x : Nat) (b : Nat) (y : Nat) (c : Nat) (z : Nat)
    : Equal Nat
        (add (add (add a x) (add b y)) (add c z))
        (add (add (add a b) c) (add (add x y) z)) =
  trans
    Nat
    (add (add (add a x) (add b y)) (add c z))
    (add (add (add a b) (add x y)) (add c z))
    (add (add (add a b) c) (add (add x y) z))
    (cong
      Nat
      Nat
      (add (add a x) (add b y))
      (add (add a b) (add x y))
      (λu. (add u (add c z)))
      (orient_swap4 a x b y))
    (orient_swap4 (add a b) (add x y) c z)

theorem orient_swap4
      (a : Nat) (x : Nat) (b : Nat) (y : Nat)
    : Equal Nat (add (add a x) (add b y)) (add (add a b) (add x y)) =
  trans
    Nat
    (add (add a x) (add b y))
    (add (add (add a x) b) y)
    (add (add a b) (add x y))
    ((proof assoc for add) (add a x) b y)
    (trans
      Nat
      (add (add (add a x) b) y)
      (add (add (add a b) x) y)
      (add (add a b) (add x y))
      (cong Nat Nat (add (add a x) b) (add (add a b) x) (λu. (add u y)) (orient_reorder a x b))
      (sym
        Nat
        (add (add a b) (add x y))
        (add (add (add a b) x) y)
        ((proof assoc for add) (add a b) x y)))

theorem orient_rotate_sum
      (p : Nat) (q : Nat) (r : Nat)
    : Equal Nat (add (add q r) p) (add (add p q) r) =
  trans
    Nat
    (add (add q r) p)
    (add p (add q r))
    (add (add p q) r)
    ((proof comm for add) (add q r) p)
    ((proof assoc for add) p q r)

pub theorem orient_scale
      (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (k : Nat)
    : Equal OrdResult
        (orient
          (MkPoint (mul xa (Suc k)) (mul ya (Suc k)))
          (MkPoint (mul xb (Suc k)) (mul yb (Suc k)))
          (MkPoint (mul xc (Suc k)) (mul yc (Suc k))))
        (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  trans
    OrdResult
    (compare
      (add
        (add (mul (mul xa (Suc k)) (mul yb (Suc k))) (mul (mul xb (Suc k)) (mul yc (Suc k))))
        (mul (mul xc (Suc k)) (mul ya (Suc k))))
      (add
        (add (mul (mul ya (Suc k)) (mul xb (Suc k))) (mul (mul yb (Suc k)) (mul xc (Suc k))))
        (mul (mul yc (Suc k)) (mul xa (Suc k)))))
    (compare
      (mul (mul (add (add (mul xa yb) (mul xb yc)) (mul xc ya)) (Suc k)) (Suc k))
      (mul (mul (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (Suc k)) (Suc k)))
    (compare
      (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
      (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
    (orient_compare_congr
      (add
        (add (mul (mul xa (Suc k)) (mul yb (Suc k))) (mul (mul xb (Suc k)) (mul yc (Suc k))))
        (mul (mul xc (Suc k)) (mul ya (Suc k))))
      (add
        (add (mul (mul ya (Suc k)) (mul xb (Suc k))) (mul (mul yb (Suc k)) (mul xc (Suc k))))
        (mul (mul yc (Suc k)) (mul xa (Suc k))))
      (mul (mul (add (add (mul xa yb) (mul xb yc)) (mul xc ya)) (Suc k)) (Suc k))
      (mul (mul (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (Suc k)) (Suc k))
      (orient_scale_sum xa yb xb yc xc ya (Suc k))
      (orient_scale_sum ya xb yb xc yc xa (Suc k)))
    (trans
      OrdResult
      (compare
        (mul (mul (add (add (mul xa yb) (mul xb yc)) (mul xc ya)) (Suc k)) (Suc k))
        (mul (mul (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (Suc k)) (Suc k)))
      (compare
        (mul (add (add (mul xa yb) (mul xb yc)) (mul xc ya)) (Suc k))
        (mul (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (Suc k)))
      (compare
        (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
        (add (add (mul ya xb) (mul yb xc)) (mul yc xa)))
      ((proof mul_cancel_r for compare)
        (mul (add (add (mul xa yb) (mul xb yc)) (mul xc ya)) (Suc k))
        (mul (add (add (mul ya xb) (mul yb xc)) (mul yc xa)) (Suc k))
        k)
      ((proof mul_cancel_r for compare)
        (add (add (mul xa yb) (mul xb yc)) (mul xc ya))
        (add (add (mul ya xb) (mul yb xc)) (mul yc xa))
        k))

theorem orient_compare_congr
      (l2 : Nat) (r2 : Nat) (l : Nat) (r : Nat) (left : Equal Nat l2 l) (right : Equal Nat r2 r)
    : Equal OrdResult (compare l2 r2) (compare l r) =
  trans
    OrdResult
    (compare l2 r2)
    (compare l r2)
    (compare l r)
    (cong Nat OrdResult l2 l (λu. (compare u r2)) left)
    (cong Nat OrdResult r2 r (compare l) right)

theorem orient_scale_sum
      (a1 : Nat) (b1 : Nat) (a2 : Nat) (b2 : Nat) (a3 : Nat) (b3 : Nat) (s : Nat)
    : Equal Nat
        (add
          (add (mul (mul a1 s) (mul b1 s)) (mul (mul a2 s) (mul b2 s)))
          (mul (mul a3 s) (mul b3 s)))
        (mul (mul (add (add (mul a1 b1) (mul a2 b2)) (mul a3 b3)) s) s) =
  trans
    Nat
    (add
      (add (mul (mul a1 s) (mul b1 s)) (mul (mul a2 s) (mul b2 s)))
      (mul (mul a3 s) (mul b3 s)))
    (add
      (add (mul (mul (mul a1 b1) s) s) (mul (mul (mul a2 b2) s) s))
      (mul (mul (mul a3 b3) s) s))
    (mul (mul (add (add (mul a1 b1) (mul a2 b2)) (mul a3 b3)) s) s)
    (orient_sum_congr
      (mul (mul a1 s) (mul b1 s))
      (mul (mul (mul a1 b1) s) s)
      (mul (mul a2 s) (mul b2 s))
      (mul (mul (mul a2 b2) s) s)
      (mul (mul a3 s) (mul b3 s))
      (mul (mul (mul a3 b3) s) s)
      (orient_scale_product a1 b1 s)
      (orient_scale_product a2 b2 s)
      (orient_scale_product a3 b3 s))
    (trans
      Nat
      (add
        (add (mul (mul (mul a1 b1) s) s) (mul (mul (mul a2 b2) s) s))
        (mul (mul (mul a3 b3) s) s))
      (mul (add (add (mul (mul a1 b1) s) (mul (mul a2 b2) s)) (mul (mul a3 b3) s)) s)
      (mul (mul (add (add (mul a1 b1) (mul a2 b2)) (mul a3 b3)) s) s)
      (orient_factor_sum (mul (mul a1 b1) s) (mul (mul a2 b2) s) (mul (mul a3 b3) s) s)
      (cong
        Nat
        Nat
        (add (add (mul (mul a1 b1) s) (mul (mul a2 b2) s)) (mul (mul a3 b3) s))
        (mul (add (add (mul a1 b1) (mul a2 b2)) (mul a3 b3)) s)
        (λu. (mul u s))
        (orient_factor_sum (mul a1 b1) (mul a2 b2) (mul a3 b3) s)))

theorem orient_factor_sum
      (p : Nat) (q : Nat) (r : Nat) (s : Nat)
    : Equal Nat (add (add (mul p s) (mul q s)) (mul r s)) (mul (add (add p q) r) s) =
  sym
    Nat
    (mul (add (add p q) r) s)
    (add (add (mul p s) (mul q s)) (mul r s))
    (trans
      Nat
      (mul (add (add p q) r) s)
      (add (mul (add p q) s) (mul r s))
      (add (add (mul p s) (mul q s)) (mul r s))
      (mul_add_distrib_l (add p q) r s)
      (cong
        Nat
        Nat
        (mul (add p q) s)
        (add (mul p s) (mul q s))
        (λu. (add u (mul r s)))
        (mul_add_distrib_l p q s)))

theorem orient_scale_product
      (x : Nat) (y : Nat) (s : Nat)
    : Equal Nat (mul (mul x s) (mul y s)) (mul (mul (mul x y) s) s) =
  trans
    Nat
    (mul (mul x s) (mul y s))
    (mul (mul (mul x s) y) s)
    (mul (mul (mul x y) s) s)
    ((proof assoc for mul) (mul x s) y s)
    (cong
      Nat
      Nat
      (mul (mul x s) y)
      (mul (mul x y) s)
      (λu. (mul u s))
      (trans
        Nat
        (mul (mul x s) y)
        (mul x (mul s y))
        (mul (mul x y) s)
        (sym Nat (mul x (mul s y)) (mul (mul x s) y) ((proof assoc for mul) x s y))
        (trans
          Nat
          (mul x (mul s y))
          (mul x (mul y s))
          (mul (mul x y) s)
          (cong Nat Nat (mul s y) (mul y s) (mul x) ((proof comm for mul) s y))
          ((proof assoc for mul) x y s))))

theorem orient_reorder
      (p : Nat) (q : Nat) (r : Nat)
    : Equal Nat (add (add p q) r) (add (add p r) q) =
  (trans
    Nat
    (add (add p q) r)
    (add p (add q r))
    (add (add p r) q)
    (sym Nat (add p (add q r)) (add (add p q) r) ((proof assoc for add) p q r))
    (trans
      Nat
      (add p (add q r))
      (add p (add r q))
      (add (add p r) q)
      (cong Nat Nat (add q r) (add r q) (add p) ((proof comm for add) q r))
      ((proof assoc for add) p r q)))
```

## Trust & derivation

`orient_translate` realizes the shared-translation law in the
[geometry spec](../../../../spec/50-stdlib/67-geometry.md) §21. It consumes the
checked Nat Order cancellation proof and adds no trusted declaration. The
translation steps and sum rearrangements are private; the public law states
only equality of the two orientation results.

The package tests check the open law, exact trusted identity equality, and
closed cases. For `(0,0), (1,0), (0,1)`, adding `(2,3)` to every point
preserves `Gt`. Moving only the first point instead changes the cross sums
to `1` and `5`, giving `Lt`.

`orient_scale` realizes the geometry spec's §22 scaling law using the checked
Nat Order positive-factor cancellation proof. Its four helper proofs are
private, and the scaling proof adds no trusted declaration.
