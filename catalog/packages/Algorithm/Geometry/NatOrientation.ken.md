# Natural-point orientation

Three natural points determine an exact turn by comparing two cross sums.

## Definition

```ken
import Algorithm.Geometry.NatBox (Point, MkPoint)

import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_result_flip)

import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add, mul)

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
