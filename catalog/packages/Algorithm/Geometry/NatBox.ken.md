# Closed boxes over natural coordinates

Points and axis-aligned boxes over `Nat` have total, exact membership and
geometry operations. Their six laws carry kernel-checked generic proofs.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A closed box stores two corners. Each coordinate is a natural number, so
membership is two inclusive comparisons per axis. Inverting either pair of
bounds represents an empty box without making operations partial or equating
all empty boxes structurally. The corners remain observable even when the
point set is empty.

## 2. Definition

The two constructors are exported separately from their carrier types so a
client can build a point and a box. The five operations and six public laws
lead the definition; the private per-axis proofs and distance helpers follow.
Intersection takes coordinatewise maximum lower and minimum upper bounds;
hull takes the opposite bounds. Distance adds two saturated absolute
coordinate differences. Every proof body below is the checked general
construction, not a finite seed in place of a proof.

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat, bool_and)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (min, max, sub)

import Core.Logic.Transport (cong, trans)

pub data Point = MkPoint Nat Nat

export MkPoint

pub data Box = MkBox Point Point

export MkBox

pub fn axis_contains (lo : Nat) (hi : Nat) (x : Nat) : Bool =
  bool_and (leq_nat lo x) (leq_nat x hi)

pub fn contains (b : Box) (p : Point) : Bool =
  match b {
    MkBox lo hi ↦
      match lo {
        MkPoint lx ly ↦
          match hi {
            MkPoint hx hy ↦
              match p {
                MkPoint px py ↦ bool_and (axis_contains lx hx px) (axis_contains ly hy py)
              }
          }
      }
  }

pub fn intersect (b : Box) (c : Box) : Box =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          MkBox
                            (MkPoint (max blx clx) (max bly cly))
                            (MkPoint (min bhx chx) (min bhy chy))
                      }
                  }
              }
          }
      }
  }

pub fn hull (b : Box) (c : Box) : Box =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          MkBox
                            (MkPoint (min blx clx) (min bly cly))
                            (MkPoint (max bhx chx) (max bhy chy))
                      }
                  }
              }
          }
      }
  }

pub fn manhattan (p : Point) (q : Point) : Nat =
  match p {
    MkPoint px py ↦
      match q {
        MkPoint qx qy ↦ add (axis_distance px qx) (axis_distance py qy)
      }
  }

pub theorem intersect_sound
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains (intersect b c) p)
      → And (IsTrue (contains b p)) (IsTrue (contains c p)) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λh.
                                and_intro
                                  (IsTrue
                                    (bool_and
                                      (axis_contains blx bhx px)
                                      (axis_contains bly bhy py)))
                                  (IsTrue
                                    (bool_and
                                      (axis_contains clx chx px)
                                      (axis_contains cly chy py)))
                                  ((proof intro for bool_and)
                                    (axis_contains blx bhx px)
                                    (axis_contains bly bhy py)
                                    (and_fst
                                      (IsTrue (axis_contains blx bhx px))
                                      (IsTrue (axis_contains clx chx px))
                                      (axis_meet_sound
                                        blx
                                        bhx
                                        clx
                                        chx
                                        px
                                        ((proof left for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h)))
                                    (and_fst
                                      (IsTrue (axis_contains bly bhy py))
                                      (IsTrue (axis_contains cly chy py))
                                      (axis_meet_sound
                                        bly
                                        bhy
                                        cly
                                        chy
                                        py
                                        ((proof right for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h))))
                                  ((proof intro for bool_and)
                                    (axis_contains clx chx px)
                                    (axis_contains cly chy py)
                                    (and_snd
                                      (IsTrue (axis_contains blx bhx px))
                                      (IsTrue (axis_contains clx chx px))
                                      (axis_meet_sound
                                        blx
                                        bhx
                                        clx
                                        chx
                                        px
                                        ((proof left for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h)))
                                    (and_snd
                                      (IsTrue (axis_contains bly bhy py))
                                      (IsTrue (axis_contains cly chy py))
                                      (axis_meet_sound
                                        bly
                                        bhy
                                        cly
                                        chy
                                        py
                                        ((proof right for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h))))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem intersect_complete
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains b p) → IsTrue (contains c p) → IsTrue (contains (intersect b c) p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λhb.
                                λhc.
                                  (proof intro for bool_and)
                                    (axis_contains (max blx clx) (min bhx chx) px)
                                    (axis_contains (max bly cly) (min bhy chy) py)
                                    (axis_meet_complete
                                      blx
                                      bhx
                                      clx
                                      chx
                                      px
                                      ((proof left for bool_and)
                                        (axis_contains blx bhx px)
                                        (axis_contains bly bhy py)
                                        hb)
                                      ((proof left for bool_and)
                                        (axis_contains clx chx px)
                                        (axis_contains cly chy py)
                                        hc))
                                    (axis_meet_complete
                                      bly
                                      bhy
                                      cly
                                      chy
                                      py
                                      ((proof right for bool_and)
                                        (axis_contains blx bhx px)
                                        (axis_contains bly bhy py)
                                        hb)
                                      ((proof right for bool_and)
                                        (axis_contains clx chx px)
                                        (axis_contains cly chy py)
                                        hc))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem hull_contains_left
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains b p) → IsTrue (contains (hull b c) p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λhb.
                                (proof intro for bool_and)
                                  (axis_contains (min blx clx) (max bhx chx) px)
                                  (axis_contains (min bly cly) (max bhy chy) py)
                                  (axis_join_left
                                    blx
                                    bhx
                                    clx
                                    chx
                                    px
                                    ((proof left for bool_and)
                                      (axis_contains blx bhx px)
                                      (axis_contains bly bhy py)
                                      hb))
                                  (axis_join_left
                                    bly
                                    bhy
                                    cly
                                    chy
                                    py
                                    ((proof right for bool_and)
                                      (axis_contains blx bhx px)
                                      (axis_contains bly bhy py)
                                      hb))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem hull_contains_right
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains c p) → IsTrue (contains (hull b c) p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λhc.
                                (proof intro for bool_and)
                                  (axis_contains (min blx clx) (max bhx chx) px)
                                  (axis_contains (min bly cly) (max bhy chy) py)
                                  (axis_join_right
                                    blx
                                    bhx
                                    clx
                                    chx
                                    px
                                    ((proof left for bool_and)
                                      (axis_contains clx chx px)
                                      (axis_contains cly chy py)
                                      hc))
                                  (axis_join_right
                                    bly
                                    bhy
                                    cly
                                    chy
                                    py
                                    ((proof right for bool_and)
                                      (axis_contains clx chx px)
                                      (axis_contains cly chy py)
                                      hc))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem manhattan_symm (p : Point) (q : Point) : Equal Nat (manhattan p q) (manhattan q p) =
  match p {
    MkPoint px py ↦
      match q {
        MkPoint qx qy ↦
          trans
            Nat
            (add (axis_distance px qx) (axis_distance py qy))
            (add (axis_distance qx px) (axis_distance py qy))
            (add (axis_distance qx px) (axis_distance qy py))
            (cong
              Nat
              Nat
              (axis_distance px qx)
              (axis_distance qx px)
              (add_right (axis_distance py qy))
              (axis_distance_symm px qx))
            (cong
              Nat
              Nat
              (axis_distance py qy)
              (axis_distance qy py)
              (add (axis_distance qx px))
              (axis_distance_symm py qy))
      }
  }

pub theorem manhattan_self (p : Point) : Equal Nat (manhattan p p) Zero =
  match p {
    MkPoint px py ↦
      add_zero_zero
        (axis_distance px px)
        (axis_distance py py)
        (axis_distance_self px)
        (axis_distance_self py)
  }

fn axis_distance (a : Nat) (b : Nat) : Nat = add (sub a b) (sub b a)

theorem axis_meet_sound
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains (max l1 l2) (min h1 h2) x)
      → And (IsTrue (axis_contains l1 h1 x)) (IsTrue (axis_contains l2 h2 x)) =
  λh.
    and_intro
      (IsTrue (axis_contains l1 h1 x))
      (IsTrue (axis_contains l2 h2 x))
      ((proof intro for bool_and)
        (leq_nat l1 x)
        (leq_nat x h1)
        ((proof trans for leq_nat)
          l1
          (max l1 l2)
          x
          ((proof left_leq for max) l1 l2)
          ((proof left for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h))
        ((proof trans for leq_nat)
          x
          (min h1 h2)
          h1
          ((proof right for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h)
          ((proof leq_left for min) h1 h2)))
      ((proof intro for bool_and)
        (leq_nat l2 x)
        (leq_nat x h2)
        ((proof trans for leq_nat)
          l2
          (max l1 l2)
          x
          ((proof right_leq for max) l1 l2)
          ((proof left for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h))
        ((proof trans for leq_nat)
          x
          (min h1 h2)
          h2
          ((proof right for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h)
          ((proof leq_right for min) h1 h2)))

theorem axis_meet_complete
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains l1 h1 x)
      → IsTrue (axis_contains l2 h2 x)
      → IsTrue (axis_contains (max l1 l2) (min h1 h2) x) =
  λa.
    λb.
      (proof intro for bool_and)
        (leq_nat (max l1 l2) x)
        (leq_nat x (min h1 h2))
        ((proof least for max)
          x
          l1
          l2
          ((proof left for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
          ((proof left for bool_and) (leq_nat l2 x) (leq_nat x h2) b))
        ((proof greatest for min)
          x
          h1
          h2
          ((proof right for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
          ((proof right for bool_and) (leq_nat l2 x) (leq_nat x h2) b))

theorem axis_join_left
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains l1 h1 x) → IsTrue (axis_contains (min l1 l2) (max h1 h2) x) =
  λa.
    (proof intro for bool_and)
      (leq_nat (min l1 l2) x)
      (leq_nat x (max h1 h2))
      ((proof trans for leq_nat)
        (min l1 l2)
        l1
        x
        ((proof leq_left for min) l1 l2)
        ((proof left for bool_and) (leq_nat l1 x) (leq_nat x h1) a))
      ((proof trans for leq_nat)
        x
        h1
        (max h1 h2)
        ((proof right for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
        ((proof left_leq for max) h1 h2))

theorem axis_join_right
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains l2 h2 x) → IsTrue (axis_contains (min l1 l2) (max h1 h2) x) =
  λa.
    (proof intro for bool_and)
      (leq_nat (min l1 l2) x)
      (leq_nat x (max h1 h2))
      ((proof trans for leq_nat)
        (min l1 l2)
        l2
        x
        ((proof leq_right for min) l1 l2)
        ((proof left for bool_and) (leq_nat l2 x) (leq_nat x h2) a))
      ((proof trans for leq_nat)
        x
        h2
        (max h1 h2)
        ((proof right for bool_and) (leq_nat l2 x) (leq_nat x h2) a)
        ((proof right_leq for max) h1 h2))

fn add_right (y : Nat) (x : Nat) : Nat = add x y

theorem axis_distance_symm
      (a : Nat) (b : Nat)
    : Equal Nat (axis_distance a b) (axis_distance b a) =
  (proof comm for add) (sub a b) (sub b a)

theorem add_zero_zero
      (x : Nat) (y : Nat)
    : Equal Nat x Zero → Equal Nat y Zero → Equal Nat (add x y) Zero =
  λhx.
    λhy.
      trans
        Nat
        (add x y)
        (add Zero y)
        Zero
        (cong Nat Nat x Zero (add_right y) hx)
        (trans Nat (add Zero y) y Zero ((proof zero_l for add) y) hy)

theorem axis_distance_self (a : Nat) : Equal Nat (axis_distance a a) Zero =
  add_zero_zero
    (sub a a)
    (sub a a)
    ((proof self_is_zero for sub) a)
    ((proof self_is_zero for sub) a)
```

## 3. Using it

Constructing a box does not require a validity premise. Both endpoints
belong to this example's closed unit square:

```ken example
const box_lower : Point = MkPoint Zero Zero

const box_upper : Point = MkPoint (Suc Zero) (Suc Zero)

const unit_box : Box = MkBox box_lower box_upper

theorem unit_box_includes_upper : Equal Bool (contains unit_box box_upper) True = Proved
```

Intersecting separated boxes still produces a `Box`: its inverted corners
express the empty result. `manhattan` on points `(1,4)` and `(3,1)` yields
five, with a contribution from each axis. The checked
[seed client](../../../../conformance/stdlib/geometry/seed-nat-box.md)
constructs these cases independently of the operations under test.

## 4. Laws & proofs

`intersect_sound` projects both coordinate witnesses from an intersection,
then transports each lower and upper comparison through the corresponding
`max` and `min` bounds. `intersect_complete` combines membership in both
inputs using the existing checked tightness laws `max::least` and
`min::greatest`; it never clones those proofs. The two hull laws separately
transport membership from the left and right inputs to their wider bounds.

`manhattan_symm` commutes each axis distance's two subtraction terms and
lifts those equalities through addition with `cong` and `trans`.
`manhattan_self` uses the existing checked subtraction self-zero proof on
each axis. The private `axis_meet_*` and `axis_join_*` lemmas keep the
coordinate arguments of the public laws explicit. All six public statements
are generic in their boxes and points, and none identifies boxes by their
sets of members.

## 5. Design notes

No validity bit or special empty constructor is needed. A box with inverted
corners remains representable and the membership predicate refuses every
point on its inverted axis. `Eq Box` compares corners structurally rather
than quotienting by membership. Hull computes a coordinatewise enclosure,
including the corners of an empty input; it does not claim a least point-set
union in the presence of differently represented empty boxes. Manhattan
distance is exact and unsigned; triangle inequality, area and signed
orientation are not part of this interface.

## 6. References

- [Axis-aligned bounding box](https://en.wikipedia.org/wiki/Minimum_bounding_box)
  — Wikipedia — geometric orientation for coordinatewise extrema.
- [Taxicab geometry](https://en.wikipedia.org/wiki/Taxicab_geometry)
  — Wikipedia — orientation for the sum of coordinate distances.
- [Natural order](../../Data/Numeric/Nat/Order.ken.md) — the checked `min`,
  `max`, `sub` and tightness-law provider used by the six laws.

## 7. Trust & derivation

1. **Contract.** [Exact geometry, §1](../../../../spec/50-stdlib/67-geometry.md)
   specifies the operations and six public proof types.
2. **Public API.** `Point`, `MkPoint`, `Box`, `MkBox`, `axis_contains`,
   `contains`, `intersect`, `hull`, `manhattan`, `intersect_sound`,
   `intersect_complete`, `hull_contains_left`, `hull_contains_right`,
   `manhattan_symm`, and `manhattan_self`. Per-axis lemmas and proof
   transport helpers remain private.
3. **Source map.** Definition contains the constructors and operations,
   followed by the checked laws and private scaffolding. Using it points to
   closed examples; Laws & proofs maps each theorem to its proof strategy.
4. **Derivation.** `Nat`, `Bool` and the ambient conjunction come from the
   compiler floor and checked classes. Arithmetic reuses the canonical
   `Data.Numeric.Nat.Arithmetic` addition and checked
   `Data.Numeric.Nat.Order` bounds and saturating subtraction. The source
   declares no `Axiom` or new primitive.
5. **Trust.** Loading this package adds zero trusted `GlobalId`s beyond the
   already-loaded provider closure; that closure may itself carry inherited
   trust. A targeted roots-loader test measures the before/after identity sets.
6. **Validation.** The
   [seed client](../../../../conformance/stdlib/geometry/seed-nat-box.md)
   supplies 22 positive closed assertions, five counterexamples and
   generic-law clients; those tests supplement rather than replace the
   checked law bodies.
