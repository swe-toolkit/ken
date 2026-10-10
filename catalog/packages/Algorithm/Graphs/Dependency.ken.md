# Finite graph reachability and dependency order

A directed graph keeps Boolean adjacency separate from finite evidence. The
result is either an order covering every vertex with all edges forward, or a
cycle built from actual edges and a return walk.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

An adjacency function alone does not certify finite search, and a sequence of
vertices alone does not witness a walk. Supply a `Finite` certificate when
asking for reachability or a dependency order. A walk carries a checked proof
for each traversed edge. An order contains distinct, complete vertices and a
checked forward direction for every edge; otherwise the result is a real
cycle. These guarantees do not require a canonical order or a shortest path.

## 2. Definition

The public declarations follow the contract's order: graph and path evidence,
predicate reachability, order predicates and certificates, then their generic
laws. Constructor exports make proof-carrying witnesses available to clients.
The private DFA encoding, search, ranking, deduplication, sorting and proofs
are given below.

```ken
import Data.Finite.Finite (Finite, MkFinite, elements, covers)

import Core.Logic.Transport (cong, sym, trans)

import Core.Classes.LawfulClasses
  (DecEq, bool_or, bool_and, leq_nat, bool_cases, or_cases, or_left, or_right)

import Data.Collections.Derived as DC

import Data.Collections.Derived
  (list_append, filter, list_elem, list_elem_head, list_elem_later, list_elem_transport)

import Core.Logic.Or (Or, Inl, Inr)

import Data.Collections.List (length)

import Data.Numeric.Nat.Order (lt_nat, leq_nat_weaken_right)

import Data.Sums.Combinators (is_some)

import Algorithm.FormalLanguages.Dfa (Dfa, MkDfa, run, run_append)

import Algorithm.FormalLanguages.Reachability
  (find_word, reachable, find_word_sound, find_word_complete)

pub data Graph (q : Type) : Type where {
  MkGraph : (q → q → Bool) → Graph q
}

export MkGraph

pub fn edge (q : Type) (g : Graph q) (s : q) (t : q) : Bool =
  match g {
    MkGraph step ↦ step s t
  }

pub data Walk (q : Type) (g : Graph q) (s : q) : q → Type where {
  WalkHere : Walk q g s s;
  WalkStep : (u : q) → (v : q) → Equal Bool (edge q g u v) True → Walk q g s u → Walk q g s v
}

export WalkHere, WalkStep

pub data Cycle (q : Type) (g : Graph q) : Type where {
  MkCycle :
    (s : q) → (next : q) → Equal Bool (edge q g s next) True → Walk q g next s → Cycle q g
}

export MkCycle

pub data ReachWitness (q : Type) (g : Graph q) (s : q) (target : q → Bool) : Type where {
  MkReach : (t : q) → Equal Bool (target t) True → Walk q g s t → ReachWitness q g s target
}

export MkReach

pub fn graph_reachable
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Bool =
  reachable q q fq fq (graph_dfa q g s) target s

pub fn graph_find_walk
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Option (ReachWitness q g s target) =
  find_walk_selected q fq g s target (graph_find_word q fq g s target) Refl

pub theorem graph_reachable_complete
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (t : q)
      (target : q → Bool)
      (walk : Walk q g s t)
    : Equal Bool (target t) True → Equal Bool (graph_reachable q fq g s target) True =
  walk_reachable q fq g s t target walk

pub theorem graph_find_walk_some
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Equal Bool (graph_reachable q fq g s target) True
      → Equal Bool
        (is_some (ReachWitness q g s target) (graph_find_walk q fq g s target))
        True =
  λreachable_true.
    trans
      Bool
      (is_some (ReachWitness q g s target) (graph_find_walk q fq g s target))
      (graph_reachable q fq g s target)
      True
      (selected_is_some q fq g s target (graph_find_word q fq g s target) Refl)
      reachable_true

pub fn contains (q : Type) (d : DecEq q) (x : q) (xs : List q) : Bool =
  match xs {
    Nil ↦ False;
    Cons y ys ↦ bool_or (d.eq x y) (contains q d x ys)
  }

pub fn before (q : Type) (d : DecEq q) (x : q) (y : q) (xs : List q) : Bool =
  match xs {
    Nil ↦ False;
    Cons head tail ↦
      match d.eq x head {
        True ↦
          match d.eq y head {
            True ↦ False;
            False ↦ contains q d y tail
          };
        False ↦
          match d.eq y head {
            True ↦ False;
            False ↦ before q d x y tail
          }
      }
  }

pub fn no_duplicates (q : Type) (d : DecEq q) (xs : List q) : Bool =
  match xs {
    Nil ↦ True;
    Cons x rest ↦
      match contains q d x rest {
        True ↦ False;
        False ↦ no_duplicates q d rest
      }
  }

pub fn all_vertices (q : Type) (d : DecEq q) (xs : List q) : Omega =
  (x : q) → Equal Bool (contains q d x xs) True

pub fn all_edges_forward (q : Type) (d : DecEq q) (g : Graph q) (xs : List q) : Omega =
  (x : q) → (y : q) → Equal Bool (edge q g x y) True → Equal Bool (before q d x y xs) True

pub data TopoOrder (q : Type) (d : DecEq q) (g : Graph q) : Type where {
  MkTopoOrder :
    (xs : List q)
    → Equal Bool (no_duplicates q d xs) True
    → all_vertices q d xs
    → all_edges_forward q d g xs
    → TopoOrder q d g
}

export MkTopoOrder

pub fn ordered_vertices
      (q : Type) (d : DecEq q) (g : Graph q) (order : TopoOrder q d g)
    : List q =
  match order {
    MkTopoOrder xs unique complete forward ↦ xs
  }

pub theorem ordered_complete
      (q : Type) (d : DecEq q) (g : Graph q) (order : TopoOrder q d g)
    : all_vertices q d (ordered_vertices q d g order) =
  match order {
    MkTopoOrder xs unique complete forward ↦ complete
  }

pub theorem ordered_unique
      (q : Type) (d : DecEq q) (g : Graph q) (order : TopoOrder q d g)
    : Equal Bool (no_duplicates q d (ordered_vertices q d g order)) True =
  match order {
    MkTopoOrder xs unique complete forward ↦ unique
  }

pub theorem ordered_forward
      (q : Type) (d : DecEq q) (g : Graph q) (order : TopoOrder q d g)
    : all_edges_forward q d g (ordered_vertices q d g order) =
  match order {
    MkTopoOrder xs unique complete forward ↦ forward
  }

pub data OrderOrCycle (q : Type) (d : DecEq q) (g : Graph q) : Type where {
  HasOrder : TopoOrder q d g → OrderOrCycle q d g;
  HasCycle : Cycle q g → OrderOrCycle q d g
}

export HasOrder, HasCycle

pub fn dependency_order_or_cycle
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    : OrderOrCycle q d g =
  match graph_cycle_decision q fq d g {
    Cycled cycle ↦ HasCycle q d g cycle;
    NoCycle no_cycles ↦
      HasOrder
        q
        d
        g
        (MkTopoOrder
          q
          d
          g
          (ordered_list q fq d g)
          (ordered_list_unique q fq d g)
          (ordered_list_covers q fq d g)
          (ordered_list_forward q fq d g no_cycles))
  }

pub theorem contains_sound
      (q : Type) (d : DecEq q) (x : q) (xs : List q)
    : Equal Bool (contains q d x xs) True → list_elem q x xs =
  match xs {
    Nil ↦ λh. absurd h;
    Cons y rest ↦
      λh.
        or_cases
          (d.eq x y)
          (contains q d x rest)
          (list_elem q x (Cons q y rest))
          h
          (λhere.
            list_elem_transport
              q
              y
              x
              (sym q x y (d.sound x y here))
              (Cons q y rest)
              (list_elem_head q y rest))
          (λlater. list_elem_later q x y rest (contains_sound q d x rest later))
  }

pub theorem contains_complete
      (q : Type) (d : DecEq q) (x : q) (xs : List q)
    : list_elem q x xs → Equal Bool (contains q d x xs) True =
  match xs {
    Nil ↦ λh. absurd h;
    Cons y rest ↦
      λh.
        elim_trunc
          (Equal Bool (contains q d x (Cons q y rest)) True)
          (λcase.
            match case {
              Inl same ↦ or_left (d.eq x y) (contains q d x rest) (d.complete x y same);
              Inr later ↦
                or_right (d.eq x y) (contains q d x rest) (contains_complete q d x rest later)
            })
          h
  }
```

## 3. Using it

A finite certificate need not have unique entries. For Boolean vertices,
`bool_finite` lists both values, and the checked Boolean equality dictionary
compares them. An edge from `True` to `False` makes the latter reachable from
the former, without making the reverse direction reachable. An order or a
cycle is chosen by the checked dependency operation, never by reading an
unchecked list as a path.

```ken example
import Data.Finite.Finite (bool_finite)

import Core.Classes.LawfulClasses (DecEq_instance_Bool, bool_not)

const forward : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          True ↦
            match v {
              True ↦ False;
              False ↦ True
            };
          False ↦ False
        })

const forward_result : OrderOrCycle Bool DecEq_instance_Bool forward =
  dependency_order_or_cycle Bool bool_finite DecEq_instance_Bool forward

const reaches_false : Bool = graph_reachable Bool bool_finite forward True (λv. bool_not v)
```

## 4. Laws & proofs

Reachability uses the existing finite-state decision with state and alphabet
both `q`. A candidate letter advances only on a checked edge; letters for
absent edges stutter. The private word-to-walk conversion discards those
stutters, so public `ReachWitness` always carries a real walk, and the
completeness proof turns any real walk back into a witness word. This is one
reachability fixed point, not another graph-specific search.

For dependency order, a finite pair scan either constructs a cycle from an
edge and a return walk or returns a checked no-cycle condition for *all*
vertices using `Finite.covers`. An edge strictly increases the count of
ancestors in the supplied finite enumeration, even when that enumeration
repeats vertices. Deduplication produces a unique list, and the imported
`Derived.sort` permutation law preserves its coverage and uniqueness. Its
sorted law combines with strict rank growth to prove that every edge points
forward. The two membership laws connect `contains` to `Derived.list_elem`
using `DecEq.sound` and `DecEq.complete`.

```ken
fn graph_step_from (q : Type) (s : q) (x : q) (b : Bool) : q =
  match b {
    True ↦ x;
    False ↦ s
  }

fn graph_dfa (q : Type) (g : Graph q) (s : q) : Dfa q q =
  MkDfa q q (λv. λnext. graph_step_from q v next (edge q g v next)) s (λv. True)

fn graph_find_word
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Option (List q) =
  find_word q q fq fq (graph_dfa q g s) target s

theorem graph_word_sound
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool) (w : List q)
    : Equal (Option (List q)) (graph_find_word q fq g s target) (Some (List q) w)
      → Equal Bool (target (run q q (graph_dfa q g s) s w)) True =
  find_word_sound q q fq fq (graph_dfa q g s) target s w

theorem graph_word_complete
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool) (w : List q)
    : Equal Bool (target (run q q (graph_dfa q g s) s w)) True
      → Equal Bool (graph_reachable q fq g s target) True =
  find_word_complete q q fq fq (graph_dfa q g s) target s w

theorem direct_step
      (q : Type) (g : Graph q) (s : q) (t : q)
    : Equal Bool (edge q g s t) True
      → Equal q (run q q (graph_dfa q g s) s (Cons q t (Nil q))) t =
  λh.
    cong
      Bool
      q
      (edge q g s t)
      True
      (λb.
        match b {
          True ↦ t;
          False ↦ s
        })
      h

fn walk_word (q : Type) (g : Graph q) (s : q) (t : q) (walk : Walk q g s t) : List q =
  match walk {
    WalkHere ↦ Nil q;
    WalkStep u v e prefix ↦ list_append q (walk_word q g s u prefix) (Cons q v (Nil q))
  }

theorem walk_word_runs
      (q : Type) (g : Graph q) (s : q) (t : q) (walk : Walk q g s t)
    : Equal q (run q q (graph_dfa q g s) s (walk_word q g s t walk)) t =
  match walk {
    WalkHere ↦ Refl;
    WalkStep u v e prefix ↦
      trans
        q
        (run
          q
          q
          (graph_dfa q g s)
          s
          (list_append q (walk_word q g s u prefix) (Cons q v (Nil q))))
        (run
          q
          q
          (graph_dfa q g s)
          (run q q (graph_dfa q g s) s (walk_word q g s u prefix))
          (Cons q v (Nil q)))
        v
        (run_append q q (graph_dfa q g s) s (walk_word q g s u prefix) (Cons q v (Nil q)))
        (trans
          q
          (run
            q
            q
            (graph_dfa q g s)
            (run q q (graph_dfa q g s) s (walk_word q g s u prefix))
            (Cons q v (Nil q)))
          (run q q (graph_dfa q g s) u (Cons q v (Nil q)))
          v
          (cong
            q
            q
            (run q q (graph_dfa q g s) s (walk_word q g s u prefix))
            u
            (λstate. run q q (graph_dfa q g s) state (Cons q v (Nil q)))
            (walk_word_runs q g s u prefix))
          (direct_step q g u v e))
  }

theorem walk_reachable
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (t : q)
      (target : q → Bool)
      (walk : Walk q g s t)
    : Equal Bool (target t) True → Equal Bool (graph_reachable q fq g s target) True =
  λhit.
    graph_word_complete
      q
      fq
      g
      s
      target
      (walk_word q g s t walk)
      (trans
        Bool
        (target (run q q (graph_dfa q g s) s (walk_word q g s t walk)))
        (target t)
        True
        (cong
          q
          Bool
          (run q q (graph_dfa q g s) s (walk_word q g s t walk))
          t
          target
          (walk_word_runs q g s t walk))
        hit)

fn walk_prepend
      (q : Type)
      (g : Graph q)
      (s : q)
      (t : q)
      (u : q)
      (first : Equal Bool (edge q g s t) True)
      (suffix : Walk q g t u)
    : Walk q g s u =
  match suffix {
    WalkHere ↦ WalkStep q g s s t first (WalkHere q g s);
    WalkStep prev end e prefix ↦
      WalkStep q g s prev end e (walk_prepend q g s t prev first prefix)
  }

fn walk_step_selected
      (q : Type) (g : Graph q) (s : q) (p : q) (x : q) (walk : Walk q g s p) (b : Bool)
    : Equal Bool (edge q g p x) b → Walk q g s (graph_step_from q p x b) =
  match b {
    True ↦ λeb. WalkStep q g s p x eb walk;
    False ↦ λeb. walk
  }

fn word_walk
      (q : Type) (g : Graph q) (s : q) (w : List q)
    : (p : q) → Walk q g s p → Walk q g s (run q q (graph_dfa q g s) p w) =
  match w {
    Nil ↦ λp. λwalk. walk;
    Cons x rest ↦
      λp.
        λwalk.
          word_walk
            q
            g
            s
            rest
            (graph_step_from q p x (edge q g p x))
            (walk_step_selected q g s p x walk (edge q g p x) Refl)
  }

fn found_walk
      (q : Type) (g : Graph q) (s : q) (w : List q)
    : Walk q g s (run q q (graph_dfa q g s) s w) =
  word_walk q g s w s (WalkHere q g s)

fn find_walk_selected
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (target : q → Bool)
      (picked : Option (List q))
    : Equal (Option (List q)) (graph_find_word q fq g s target) picked
      → Option (ReachWitness q g s target) =
  match picked {
    None ↦ λh. None (ReachWitness q g s target);
    Some w ↦
      λh.
        Some
          (ReachWitness q g s target)
          (MkReach
            q
            g
            s
            target
            (run q q (graph_dfa q g s) s w)
            (graph_word_sound q fq g s target w h)
            (found_walk q g s w))
  }

theorem selected_is_some
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (target : q → Bool)
      (picked : Option (List q))
    : (picked_eq : Equal (Option (List q)) (graph_find_word q fq g s target) picked)
      → Equal Bool
        (is_some
          (ReachWitness q g s target)
          (find_walk_selected q fq g s target picked picked_eq))
        (is_some (List q) picked) =
  match picked {
    None ↦ λh. Proved;
    Some w ↦ λh. Proved
  }

fn require_some (a : Type) (picked : Option a) : Equal Bool (is_some a picked) True → a =
  match picked {
    None ↦ λh. absurd h;
    Some value ↦ λh. value
  }

fn walk_transport_end
      (q : Type)
      (g : Graph q)
      (s : q)
      (t : q)
      (u : q)
      (same : Equal q t u)
      (walk : Walk q g s t)
    : Walk q g s u =
  J (λpoint _. Walk q g s point) walk same

fn named (q : Type) (d : DecEq q) (x : q) (v : q) : Bool = d.eq v x

theorem edge_reach_trans
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (z : q) (x : q) (y : q)
    : Equal Bool (edge q g x y) True
      → Equal Bool (graph_reachable q fq g z (named q d x)) True
      → Equal Bool (graph_reachable q fq g z (named q d y)) True =
  λedge_xy.
    λreach_zx.
      let witness =
        require_some
          (ReachWitness q g z (named q d x))
          (graph_find_walk q fq g z (named q d x))
          (graph_find_walk_some q fq g z (named q d x) reach_zx)
      in
        match witness {
          MkReach t hit walk ↦
            walk_reachable
              q
              fq
              g
              z
              y
              (named q d y)
              (WalkStep q g z x y edge_xy (walk_transport_end q g z t x (d.sound t x hit) walk))
              (d.complete y y Refl)
        }

theorem graph_reachable_refl
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (s : q)
    : Equal Bool (graph_reachable q fq g s (named q d s)) True =
  walk_reachable q fq g s s (named q d s) (WalkHere q g s) (d.complete s s Refl)

fn cycle_from_hit
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (v : q)
      (u : q)
      (first : Equal Bool (edge q g v u) True)
      (back : Equal Bool (graph_reachable q fq g u (named q d v)) True)
    : Cycle q g =
  let witness =
    require_some
      (ReachWitness q g u (named q d v))
      (graph_find_walk q fq g u (named q d v))
      (graph_find_walk_some q fq g u (named q d v) back)
  in
    match witness {
      MkReach t hit walk ↦
        MkCycle q g v u first (walk_transport_end q g u t v (d.sound t v hit) walk)
    }

fn no_cycle_at (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (v : q) (u : q) : Omega =
  Equal Bool (edge q g v u) True → Equal Bool (graph_reachable q fq g u (named q d v)) False

theorem no_cycle_edge_false
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (v : q)
      (u : q)
      (edge_false : Equal Bool (edge q g v u) False)
    : no_cycle_at q fq d g v u =
  λedge_true.
    absurd
      (trans Bool True (edge q g v u) False (sym Bool (edge q g v u) True edge_true) edge_false)

theorem no_cycle_list_cons
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (v : q)
      (u : q)
      (rest : List q)
      (first : no_cycle_at q fq d g v u)
      (later : (x : q) → list_elem q x rest → no_cycle_at q fq d g v x)
      (x : q)
    : list_elem q x (Cons q u rest) → no_cycle_at q fq d g v x =
  λmember.
    elim_trunc
      (no_cycle_at q fq d g v x)
      (λcase.
        match case {
          Inl same ↦ J (λv2 _. no_cycle_at q fq d g v v2) first (sym q x u same);
          Inr tail ↦ later x tail
        })
      member

data RowResult
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (v : q) (ys : List q)
    : Type
    where {
  RowCycle : Cycle q g → RowResult q fq d g v ys;
  RowClear : ((x : q) → list_elem q x ys → no_cycle_at q fq d g v x) → RowResult q fq d g v ys
}

fn row_scan
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (v : q) (ys : List q)
    : RowResult q fq d g v ys =
  match ys {
    Nil ↦ RowClear q fq d g v (Nil q) (λx. λmember. absurd member);
    Cons u rest ↦ row_scan_edge q fq d g v u rest (edge q g v u) Refl
  }

fn row_scan_edge
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (v : q)
      (u : q)
      (rest : List q)
      (b : Bool)
    : Equal Bool (edge q g v u) b → RowResult q fq d g v (Cons q u rest) =
  match b {
    True ↦
      λeb. row_scan_reach q fq d g v u rest eb (graph_reachable q fq g u (named q d v)) Refl;
    False ↦
      λeb.
        match row_scan q fq d g v rest {
          RowCycle cycle ↦ RowCycle q fq d g v (Cons q u rest) cycle;
          RowClear later ↦
            RowClear
              q
              fq
              d
              g
              v
              (Cons q u rest)
              (no_cycle_list_cons q fq d g v u rest (no_cycle_edge_false q fq d g v u eb) later)
        }
  }

fn row_scan_reach
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (v : q)
      (u : q)
      (rest : List q)
      (edge_true : Equal Bool (edge q g v u) True)
      (b : Bool)
    : Equal Bool (graph_reachable q fq g u (named q d v)) b
      → RowResult q fq d g v (Cons q u rest) =
  match b {
    True ↦ λrb. RowCycle q fq d g v (Cons q u rest) (cycle_from_hit q fq d g v u edge_true rb);
    False ↦
      λrb.
        match row_scan q fq d g v rest {
          RowCycle cycle ↦ RowCycle q fq d g v (Cons q u rest) cycle;
          RowClear later ↦
            RowClear
              q
              fq
              d
              g
              v
              (Cons q u rest)
              (no_cycle_list_cons q fq d g v u rest (λedge_h. rb) later)
        }
  }

data ScanResult
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (xs : List q) (ys : List q)
    : Type
    where {
  ScanCycle : Cycle q g → ScanResult q fq d g xs ys;
  ScanClear :
    ((v : q) → list_elem q v xs → (u : q) → list_elem q u ys → no_cycle_at q fq d g v u)
    → ScanResult q fq d g xs ys
}

theorem scan_clear_cons
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (x : q)
      (rest : List q)
      (ys : List q)
      (first : (u : q) → list_elem q u ys → no_cycle_at q fq d g x u)
      (later : (v : q)
        → list_elem
        q
        v
        rest
        → (u : q)
        → list_elem
        q
        u
        ys
        → no_cycle_at
        q
        fq
        d
        g
        v
        u)
      (v : q)
    : list_elem q v (Cons q x rest) → (u : q) → list_elem q u ys → no_cycle_at q fq d g v u =
  λmember.
    elim_trunc
      ((u : q) → list_elem q u ys → no_cycle_at q fq d g v u)
      (λcase.
        match case {
          Inl same ↦
            J
              (λv2 _. (u : q) → list_elem q u ys → no_cycle_at q fq d g v2 u)
              first
              (sym q v x same);
          Inr tail ↦ later v tail
        })
      member

fn scan_pairs
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (xs : List q) (ys : List q)
    : ScanResult q fq d g xs ys =
  match xs {
    Nil ↦ ScanClear q fq d g (Nil q) ys (λv. λmember. absurd member);
    Cons v rest ↦
      match row_scan q fq d g v ys {
        RowCycle cycle ↦ ScanCycle q fq d g (Cons q v rest) ys cycle;
        RowClear first ↦
          match scan_pairs q fq d g rest ys {
            ScanCycle cycle ↦ ScanCycle q fq d g (Cons q v rest) ys cycle;
            ScanClear later ↦
              ScanClear
                q
                fq
                d
                g
                (Cons q v rest)
                ys
                (scan_clear_cons q fq d g v rest ys first later)
          }
      }
  }

data GraphCycleDecision (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) : Type where {
  Cycled : Cycle q g → GraphCycleDecision q fq d g;
  NoCycle : ((v : q) → (u : q) → no_cycle_at q fq d g v u) → GraphCycleDecision q fq d g
}

fn graph_cycle_decision
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    : GraphCycleDecision q fq d g =
  match scan_pairs q fq d g (elements q fq) (elements q fq) {
    ScanCycle cycle ↦ Cycled q fq d g cycle;
    ScanClear clear ↦ NoCycle q fq d g (λv. λu. clear v (covers q fq v) u (covers q fq u))
  }

fn next_count (b : Bool) (n : Nat) : Nat =
  match b {
    True ↦ Suc n;
    False ↦ n
  }

fn step_list (a : Type) (x : a) (xs : List a) (b : Bool) : List a =
  match b {
    True ↦ Cons a x xs;
    False ↦ xs
  }

theorem step_list_length
      (a : Type) (x : a) (xs : List a) (b : Bool)
    : Equal Nat (length a (step_list a x xs b)) (next_count b (length a xs)) =
  match b {
    True ↦ Refl;
    False ↦ Refl
  }

theorem filter_length_cons
      (q : Type) (p : q → Bool) (x : q) (xs : List q)
    : Equal Nat
        (length q (filter q p (Cons q x xs)))
        (next_count (p x) (length q (filter q p xs))) =
  step_list_length q x (filter q p xs) (p x)

fn true_count (q : Type) (p : q → Bool) (xs : List q) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons x tail ↦ next_count (p x) (true_count q p tail)
  }

theorem true_count_filter_length
      (q : Type) (p : q → Bool) (xs : List q)
    : Equal Nat (true_count q p xs) (length q (filter q p xs)) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      trans
        Nat
        (next_count (p x) (true_count q p rest))
        (next_count (p x) (length q (filter q p rest)))
        (length q (filter q p (Cons q x rest)))
        (cong
          Nat
          Nat
          (true_count q p rest)
          (length q (filter q p rest))
          (next_count (p x))
          (true_count_filter_length q p rest))
        (sym
          Nat
          (length q (filter q p (Cons q x rest)))
          (next_count (p x) (length q (filter q p rest)))
          (filter_length_cons q p x rest))
  }

theorem step_count_leq
      (pb : Bool) (qb : Bool) (n : Nat) (m : Nat)
    : (Equal Bool pb True → Equal Bool qb True)
      → Equal Bool (leq_nat n m) True
      → Equal Bool (leq_nat (next_count pb n) (next_count qb m)) True =
  match pb {
    True ↦
      match qb {
        True ↦ λsubset. λle. le;
        False ↦ λsubset. λle. absurd (subset Proved)
      };
    False ↦
      match qb {
        True ↦ λsubset. λle. leq_nat_weaken_right n m le;
        False ↦ λsubset. λle. le
      }
  }

theorem true_count_leq
      (q : Type)
      (p : q → Bool)
      (r : q → Bool)
      (subset : (x : q) → Equal Bool (p x) True → Equal Bool (r x) True)
      (xs : List q)
    : Equal Bool (leq_nat (true_count q p xs) (true_count q r xs)) True =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      step_count_leq
        (p x)
        (r x)
        (true_count q p rest)
        (true_count q r rest)
        (subset x)
        (true_count_leq q p r subset rest)
  }

theorem leq_to_lt_suc
      (n : Nat)
    : (m : Nat) → Equal Bool (leq_nat n m) True → Equal Bool (lt_nat n (Suc m)) True =
  match n {
    Zero ↦ λm. λh. Proved;
    Suc k ↦
      λm.
        match m {
          Zero ↦ λh. absurd h;
          Suc j ↦ λh. leq_to_lt_suc k j h
        }
  }

theorem lt_to_leq
      (n : Nat)
    : (m : Nat) → Equal Bool (lt_nat n m) True → Equal Bool (leq_nat n m) True =
  match n {
    Zero ↦ λm. λlt. Proved;
    Suc k ↦
      λm.
        match m {
          Zero ↦ λlt. absurd lt;
          Suc j ↦ λlt. lt_to_leq k j lt
        }
  }

theorem lt_right_suc
      (n : Nat) (m : Nat)
    : Equal Bool (lt_nat n m) True → Equal Bool (lt_nat n (Suc m)) True =
  λlt. leq_to_lt_suc n m (lt_to_leq n m lt)

theorem step_count_strict_tail
      (pb : Bool) (qb : Bool) (n : Nat) (m : Nat)
    : (Equal Bool pb True → Equal Bool qb True)
      → Equal Bool (lt_nat n m) True
      → Equal Bool (lt_nat (next_count pb n) (next_count qb m)) True =
  match pb {
    True ↦
      match qb {
        True ↦ λsubset. λlt. lt;
        False ↦ λsubset. λlt. absurd (subset Proved)
      };
    False ↦
      match qb {
        True ↦ λsubset. λlt. lt_right_suc n m lt;
        False ↦ λsubset. λlt. lt
      }
  }

theorem step_count_strict_head
      (pb : Bool) (qb : Bool) (n : Nat) (m : Nat)
    : Equal Bool pb False
      → Equal Bool qb True
      → Equal Bool (leq_nat n m) True
      → Equal Bool (lt_nat (next_count pb n) (next_count qb m)) True =
  match pb {
    True ↦ λpf. λqt. λle. absurd pf;
    False ↦
      match qb {
        True ↦ λpf. λqt. λle. leq_to_lt_suc n m le;
        False ↦ λpf. λqt. λle. absurd qt
      }
  }

theorem true_count_strict
      (q : Type)
      (p : q → Bool)
      (r : q → Bool)
      (subset : (v : q) → Equal Bool (p v) True → Equal Bool (r v) True)
      (xs : List q)
      (witness : q)
    : list_elem q witness xs
      → Equal Bool (p witness) False
      → Equal Bool (r witness) True
      → Equal Bool (lt_nat (true_count q p xs) (true_count q r xs)) True =
  match xs {
    Nil ↦ λmember. λpf. λqt. absurd member;
    Cons y rest ↦
      λmember.
        λpf.
          λqt.
            elim_trunc
              (Equal
                Bool
                (lt_nat (true_count q p (Cons q y rest)) (true_count q r (Cons q y rest)))
                True)
              (λcase.
                match case {
                  Inl same ↦
                    step_count_strict_head
                      (p y)
                      (r y)
                      (true_count q p rest)
                      (true_count q r rest)
                      (trans
                        Bool
                        (p y)
                        (p witness)
                        False
                        (sym Bool (p witness) (p y) (cong q Bool witness y p same))
                        pf)
                      (trans
                        Bool
                        (r y)
                        (r witness)
                        True
                        (sym Bool (r witness) (r y) (cong q Bool witness y r same))
                        qt)
                      (true_count_leq q p r subset rest);
                  Inr later ↦
                    step_count_strict_tail
                      (p y)
                      (r y)
                      (true_count q p rest)
                      (true_count q r rest)
                      (subset y)
                      (true_count_strict q p r subset rest witness later pf qt)
                })
              member
  }

theorem length_filter_strict
      (q : Type)
      (p : q → Bool)
      (r : q → Bool)
      (subset : (v : q) → Equal Bool (p v) True → Equal Bool (r v) True)
      (xs : List q)
      (witness : q)
    : list_elem q witness xs
      → Equal Bool (p witness) False
      → Equal Bool (r witness) True
      → Equal Bool (lt_nat (length q (filter q p xs)) (length q (filter q r xs))) True =
  λmember.
    λpf.
      λqt.
        let
          left = length q (filter q p xs);
          right = length q (filter q r xs);
          pc = true_count q p xs;
          rc = true_count q r xs
        in
          trans
            Bool
            (lt_nat left right)
            (lt_nat left rc)
            True
            (cong
              Nat
              Bool
              right
              rc
              (λv. lt_nat left v)
              (sym Nat rc right (true_count_filter_length q r xs)))
            (trans
              Bool
              (lt_nat left rc)
              (lt_nat pc rc)
              True
              (cong
                Nat
                Bool
                left
                pc
                (λv. lt_nat v rc)
                (sym Nat pc left (true_count_filter_length q p xs)))
              (true_count_strict q p r subset xs witness member pf qt))

fn ancestor (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (y : q) (z : q) : Bool =
  graph_reachable q fq g z (named q d y)

fn rank (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (y : q) : Nat =
  length q (filter q (ancestor q fq d g y) (elements q fq))

theorem edge_rank_strict
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (no_cycles : (v : q) → (u : q) → no_cycle_at q fq d g v u)
      (x : q)
      (y : q)
    : Equal Bool (edge q g x y) True
      → Equal Bool (lt_nat (rank q fq d g x) (rank q fq d g y)) True =
  λedge_xy.
    length_filter_strict
      q
      (ancestor q fq d g x)
      (ancestor q fq d g y)
      (λz. λreach_zx. edge_reach_trans q fq d g z x y edge_xy reach_zx)
      (elements q fq)
      y
      (covers q fq y)
      (no_cycles x y edge_xy)
      (graph_reachable_refl q fq d g y)

fn dedup_step (q : Type) (x : q) (tail : List q) (duplicate : Bool) : List q =
  match duplicate {
    True ↦ tail;
    False ↦ Cons q x tail
  }

fn dedup (q : Type) (d : DecEq q) (xs : List q) : List q =
  match xs {
    Nil ↦ Nil q;
    Cons x rest ↦ dedup_step q x (dedup q d rest) (contains q d x (dedup q d rest))
  }

fn nodup_step (duplicate : Bool) (tail_ok : Bool) : Bool =
  match duplicate {
    True ↦ False;
    False ↦ tail_ok
  }

theorem dedup_step_unique
      (q : Type) (d : DecEq q) (x : q) (tail : List q) (b : Bool)
    : Equal Bool (contains q d x tail) b
      → Equal Bool (no_duplicates q d tail) True
      → Equal Bool (no_duplicates q d (dedup_step q x tail b)) True =
  match b {
    True ↦ λeq. λvalid. valid;
    False ↦
      λeq.
        λvalid.
          trans
            Bool
            (no_duplicates q d (Cons q x tail))
            (no_duplicates q d tail)
            True
            (cong
              Bool
              Bool
              (contains q d x tail)
              False
              (λt. nodup_step t (no_duplicates q d tail))
              eq)
            valid
  }

theorem dedup_unique
      (q : Type) (d : DecEq q) (xs : List q)
    : Equal Bool (no_duplicates q d (dedup q d xs)) True =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      dedup_step_unique
        q
        d
        x
        (dedup q d rest)
        (contains q d x (dedup q d rest))
        Refl
        (dedup_unique q d rest)
  }

theorem dedup_step_head
      (q : Type) (d : DecEq q) (head : q) (tail : List q) (b : Bool)
    : Equal Bool (contains q d head tail) b → list_elem q head (dedup_step q head tail b) =
  match b {
    True ↦ λpresent. contains_sound q d head tail present;
    False ↦ λabsent. list_elem_head q head tail
  }

theorem dedup_step_later
      (q : Type) (x : q) (head : q) (tail : List q) (b : Bool)
    : list_elem q x tail → list_elem q x (dedup_step q head tail b) =
  match b {
    True ↦ λmember. member;
    False ↦ λmember. list_elem_later q x head tail member
  }

theorem dedup_preserves_member
      (q : Type) (d : DecEq q) (x : q) (xs : List q)
    : list_elem q x xs → list_elem q x (dedup q d xs) =
  match xs {
    Nil ↦ λmember. absurd member;
    Cons head rest ↦
      λmember.
        elim_trunc
          (list_elem q x (dedup q d (Cons q head rest)))
          (λcase.
            match case {
              Inl same ↦
                list_elem_transport
                  q
                  head
                  x
                  (sym q x head same)
                  (dedup q d (Cons q head rest))
                  (dedup_step_head
                    q
                    d
                    head
                    (dedup q d rest)
                    (contains q d head (dedup q d rest))
                    Refl);
              Inr later ↦
                dedup_step_later
                  q
                  x
                  head
                  (dedup q d rest)
                  (contains q d head (dedup q d rest))
                  (dedup_preserves_member q d x rest later)
            })
          member
  }

theorem dedup_covers
      (q : Type) (fq : Finite q) (d : DecEq q) (x : q)
    : Equal Bool (contains q d x (dedup q d (elements q fq))) True =
  contains_complete
    q
    d
    x
    (dedup q d (elements q fq))
    (dedup_preserves_member q d x (elements q fq) (covers q fq x))

fn is_positive (n : Nat) : Bool =
  match n {
    Zero ↦ False;
    Suc k ↦ True
  }

theorem is_positive_suc (n : Nat) : Equal Bool (is_positive (Suc n)) True = Proved

theorem positive_count_step_true
      (n : Nat)
    : Equal Bool (is_positive (next_count True n)) (bool_or True (is_positive n)) =
  is_positive_suc n

theorem positive_count_step_false
      (n : Nat)
    : Equal Bool (is_positive (next_count False n)) (bool_or False (is_positive n)) =
  cong Nat Bool n n is_positive Refl

theorem positive_count_step
      (b : Bool) (n : Nat)
    : Equal Bool (is_positive (next_count b n)) (bool_or b (is_positive n)) =
  match b {
    True ↦ positive_count_step_true n;
    False ↦ positive_count_step_false n
  }

fn count_at (q : Type) (d : DecEq q) (x : q) (xs : List q) : Nat = DC.count q d.eq x xs

theorem contains_count
      (q : Type) (d : DecEq q) (x : q) (xs : List q)
    : Equal Bool (contains q d x xs) (is_positive (count_at q d x xs)) =
  match xs {
    Nil ↦ Proved;
    Cons y rest ↦
      trans
        Bool
        (contains q d x (Cons q y rest))
        (bool_or (d.eq x y) (is_positive (count_at q d x rest)))
        (is_positive (count_at q d x (Cons q y rest)))
        (cong
          Bool
          Bool
          (contains q d x rest)
          (is_positive (count_at q d x rest))
          (λv. bool_or (d.eq x y) v)
          (contains_count q d x rest))
        (sym
          Bool
          (is_positive (count_at q d x (Cons q y rest)))
          (bool_or (d.eq x y) (is_positive (count_at q d x rest)))
          (positive_count_step (d.eq x y) (count_at q d x rest)))
  }

theorem contains_perm
      (q : Type)
      (d : DecEq q)
      (xs : List q)
      (ys : List q)
      (permutation : DC.Perm q d.eq xs ys)
      (x : q)
    : Equal Bool (contains q d x xs) (contains q d x ys) =
  trans
    Bool
    (contains q d x xs)
    (is_positive (count_at q d x xs))
    (contains q d x ys)
    (contains_count q d x xs)
    (trans
      Bool
      (is_positive (count_at q d x xs))
      (is_positive (count_at q d x ys))
      (contains q d x ys)
      (cong Nat Bool (count_at q d x xs) (count_at q d x ys) is_positive (permutation x))
      (sym
        Bool
        (contains q d x ys)
        (is_positive (count_at q d x ys))
        (contains_count q d x ys)))

data UniqueParts (q : Type) (d : DecEq q) (head : q) (tail : List q) : Type where {
  MkUniqueParts :
    Equal Bool (contains q d head tail) False
    → Equal Bool (no_duplicates q d tail) True
    → UniqueParts q d head tail
}

fn unique_parts_at
      (q : Type) (d : DecEq q) (head : q) (tail : List q) (b : Bool)
    : Equal Bool (contains q d head tail) b
      → Equal Bool (no_duplicates q d (Cons q head tail)) True
      → UniqueParts q d head tail =
  match b {
    True ↦
      λh.
        λvalid.
          absurd
            (trans
              Bool
              False
              (no_duplicates q d (Cons q head tail))
              True
              (sym
                Bool
                (no_duplicates q d (Cons q head tail))
                False
                (cong
                  Bool
                  Bool
                  (contains q d head tail)
                  True
                  (λv. nodup_step v (no_duplicates q d tail))
                  h))
              valid);
    False ↦
      λh.
        λvalid.
          MkUniqueParts
            q
            d
            head
            tail
            h
            (trans
              Bool
              (no_duplicates q d tail)
              (no_duplicates q d (Cons q head tail))
              True
              (sym
                Bool
                (no_duplicates q d (Cons q head tail))
                (no_duplicates q d tail)
                (cong
                  Bool
                  Bool
                  (contains q d head tail)
                  False
                  (λv. nodup_step v (no_duplicates q d tail))
                  h))
              valid)
  }

fn unique_parts
      (q : Type)
      (d : DecEq q)
      (head : q)
      (tail : List q)
      (valid : Equal Bool (no_duplicates q d (Cons q head tail)) True)
    : UniqueParts q d head tail =
  unique_parts_at q d head tail (contains q d head tail) Refl valid

theorem positive_false_zero (n : Nat) : Equal Bool (is_positive n) False → Equal Nat n Zero =
  match n {
    Zero ↦ λh. Proved;
    Suc k ↦ λh. absurd h
  }

theorem absent_count_zero
      (q : Type) (d : DecEq q) (x : q) (xs : List q)
    : Equal Bool (contains q d x xs) False → Equal Nat (count_at q d x xs) Zero =
  λabsent.
    positive_false_zero
      (count_at q d x xs)
      (trans
        Bool
        (is_positive (count_at q d x xs))
        (contains q d x xs)
        False
        (sym
          Bool
          (contains q d x xs)
          (is_positive (count_at q d x xs))
          (contains_count q d x xs))
        absent)

theorem bound_one_step
      (b : Bool) (n : Nat)
    : (Equal Bool b True → Equal Nat n Zero)
      → Equal Bool (leq_nat n (Suc Zero)) True
      → Equal Bool (leq_nat (next_count b n) (Suc Zero)) True =
  match b {
    True ↦
      λzero.
        λbound.
          trans
            Bool
            (leq_nat (Suc n) (Suc Zero))
            (leq_nat (Suc Zero) (Suc Zero))
            True
            (cong Nat Bool n Zero (λv. leq_nat (Suc v) (Suc Zero)) (zero Proved))
            Proved;
    False ↦ λzero. λbound. bound
  }

theorem count_other_zero
      (q : Type) (d : DecEq q) (x : q) (head : q) (tail : List q)
    : Equal Bool (contains q d head tail) False
      → Equal Bool (d.eq x head) True
      → Equal Nat (count_at q d x tail) Zero =
  λabsent.
    λsame.
      trans
        Nat
        (count_at q d x tail)
        (count_at q d head tail)
        Zero
        (cong q Nat x head (λv. count_at q d v tail) (d.sound x head same))
        (absent_count_zero q d head tail absent)

theorem nodup_count_leq_one
      (q : Type) (d : DecEq q) (xs : List q) (x : q)
    : Equal Bool (no_duplicates q d xs) True
      → Equal Bool (leq_nat (count_at q d x xs) (Suc Zero)) True =
  match xs {
    Nil ↦ λvalid. Proved;
    Cons head tail ↦
      λvalid.
        match unique_parts q d head tail valid {
          MkUniqueParts absent tail_valid ↦
            bound_one_step
              (d.eq x head)
              (count_at q d x tail)
              (count_other_zero q d x head tail absent)
              (nodup_count_leq_one q d tail x tail_valid)
        }
  }

theorem count_self_cons
      (q : Type) (d : DecEq q) (head : q) (tail : List q)
    : Equal Nat (count_at q d head (Cons q head tail)) (Suc (count_at q d head tail)) =
  cong
    Bool
    Nat
    (d.eq head head)
    True
    (λv. next_count v (count_at q d head tail))
    (d.complete head head Refl)

theorem positive_suc_bound_absurd
      (n : Nat)
    : Equal Bool (is_positive n) True → Equal Bool (leq_nat (Suc n) (Suc Zero)) True → Bottom =
  match n {
    Zero ↦ λpositive. λbounded. absurd positive;
    Suc k ↦ λpositive. λbounded. absurd bounded
  }

theorem head_absent_at
      (q : Type) (d : DecEq q) (head : q) (tail : List q) (b : Bool)
    : Equal Bool (contains q d head tail) b
      → Equal Bool (leq_nat (count_at q d head (Cons q head tail)) (Suc Zero)) True
      → Equal Bool (contains q d head tail) False =
  match b {
    True ↦
      λpresent.
        λbound.
          absurd
            (positive_suc_bound_absurd
              (count_at q d head tail)
              (trans
                Bool
                (is_positive (count_at q d head tail))
                (contains q d head tail)
                True
                (sym
                  Bool
                  (contains q d head tail)
                  (is_positive (count_at q d head tail))
                  (contains_count q d head tail))
                present)
              (trans
                Bool
                (leq_nat (Suc (count_at q d head tail)) (Suc Zero))
                (leq_nat (count_at q d head (Cons q head tail)) (Suc Zero))
                True
                (cong
                  Nat
                  Bool
                  (Suc (count_at q d head tail))
                  (count_at q d head (Cons q head tail))
                  (λn. leq_nat n (Suc Zero))
                  (sym
                    Nat
                    (count_at q d head (Cons q head tail))
                    (Suc (count_at q d head tail))
                    (count_self_cons q d head tail)))
                bound));
    False ↦ λabsent. λbound. absent
  }

theorem head_absent_from_bound
      (q : Type) (d : DecEq q) (head : q) (tail : List q)
    : Equal Bool (leq_nat (count_at q d head (Cons q head tail)) (Suc Zero)) True
      → Equal Bool (contains q d head tail) False =
  head_absent_at q d head tail (contains q d head tail) Refl

theorem step_bound_tail
      (b : Bool) (n : Nat)
    : Equal Bool (leq_nat (next_count b n) (Suc Zero)) True
      → Equal Bool (leq_nat n (Suc Zero)) True =
  match b {
    True ↦ λbound. leq_nat_weaken_right n Zero bound;
    False ↦ λbound. bound
  }

theorem count_bound_nodup
      (q : Type) (d : DecEq q) (xs : List q)
    : ((x : q) → Equal Bool (leq_nat (count_at q d x xs) (Suc Zero)) True)
      → Equal Bool (no_duplicates q d xs) True =
  match xs {
    Nil ↦ λall. Proved;
    Cons head tail ↦
      λall.
        let
          head_absent = head_absent_from_bound q d head tail (all head);
          tail_valid =
            count_bound_nodup
              q
              d
              tail
              (λx. step_bound_tail (d.eq x head) (count_at q d x tail) (all x))
        in
          trans
            Bool
            (no_duplicates q d (Cons q head tail))
            (no_duplicates q d tail)
            True
            (cong
              Bool
              Bool
              (contains q d head tail)
              False
              (λb. nodup_step b (no_duplicates q d tail))
              head_absent)
            tail_valid
  }

theorem nodup_perm
      (q : Type) (d : DecEq q) (xs : List q) (ys : List q) (permutation : DC.Perm q d.eq xs ys)
    : Equal Bool (no_duplicates q d xs) True → Equal Bool (no_duplicates q d ys) True =
  λunique.
    count_bound_nodup
      q
      d
      ys
      (λx.
        trans
          Bool
          (leq_nat (count_at q d x ys) (Suc Zero))
          (leq_nat (count_at q d x xs) (Suc Zero))
          True
          (cong
            Nat
            Bool
            (count_at q d x ys)
            (count_at q d x xs)
            (λn. leq_nat n (Suc Zero))
            (sym Nat (count_at q d x xs) (count_at q d x ys) (permutation x)))
          (nodup_count_leq_one q d xs x unique))

fn rank_leq (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (x : q) (y : q) : Bool =
  leq_nat (rank q fq d g x) (rank q fq d g y)

fn ordered_list (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) : List q =
  DC.sort q (rank_leq q fq d g) (dedup q d (elements q fq))

theorem ordered_list_sorted
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    : is_sorted q (rank_leq q fq d g) (ordered_list q fq d g) =
  DC.sort::sorted
    q
    (rank_leq q fq d g)
    (λx. λy. (proof total for leq_nat) (rank q fq d g x) (rank q fq d g y))
    (dedup q d (elements q fq))

theorem ordered_list_unique
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    : Equal Bool (no_duplicates q d (ordered_list q fq d g)) True =
  nodup_perm
    q
    d
    (dedup q d (elements q fq))
    (ordered_list q fq d g)
    (DC.sort::perm q (rank_leq q fq d g) (dedup q d (elements q fq)) d.eq)
    (dedup_unique q d (elements q fq))

theorem ordered_list_covers
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    : all_vertices q d (ordered_list q fq d g) =
  λx.
    trans
      Bool
      (contains q d x (ordered_list q fq d g))
      (contains q d x (dedup q d (elements q fq)))
      True
      (sym
        Bool
        (contains q d x (dedup q d (elements q fq)))
        (contains q d x (ordered_list q fq d g))
        (contains_perm
          q
          d
          (dedup q d (elements q fq))
          (ordered_list q fq d g)
          (DC.sort::perm q (rank_leq q fq d g) (dedup q d (elements q fq)) d.eq)
          x))
      (dedup_covers q fq d x)

theorem sorted_head_le_member
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (head : q) (rest : List q) (x : q)
    : is_sorted q (rank_leq q fq d g) (Cons q head rest)
      → list_elem q x rest
      → Equal Bool (rank_leq q fq d g head x) True =
  match rest {
    Nil ↦ λsorted. λmember. absurd member;
    Cons next tail ↦
      λsorted.
        λmember.
          let
            le_head_next : Equal Bool (rank_leq q fq d g head next) True =
              and_fst
                (Equal Bool (rank_leq q fq d g head next) True)
                (is_sorted q (rank_leq q fq d g) (Cons q next tail))
                sorted;
            sorted_tail : is_sorted q (rank_leq q fq d g) (Cons q next tail) =
              and_snd
                (Equal Bool (rank_leq q fq d g head next) True)
                (is_sorted q (rank_leq q fq d g) (Cons q next tail))
                sorted
          in
            elim_trunc
              (Equal Bool (rank_leq q fq d g head x) True)
              (λcase.
                match case {
                  Inl same ↦
                    trans
                      Bool
                      (rank_leq q fq d g head x)
                      (rank_leq q fq d g head next)
                      True
                      (cong q Bool x next (λv. rank_leq q fq d g head v) same)
                      le_head_next;
                  Inr later ↦
                    (proof trans for leq_nat)
                      (rank q fq d g head)
                      (rank q fq d g next)
                      (rank q fq d g x)
                      le_head_next
                      (sorted_head_le_member q fq d g next tail x sorted_tail later)
                })
              member
  }

theorem successor_not_self_leq (n : Nat) : Equal Bool (leq_nat (Suc n) n) True → Bottom =
  match n {
    Zero ↦ λh. absurd h;
    Suc k ↦ λh. successor_not_self_leq k h
  }

theorem strict_excludes_reverse
      (a : Nat) (b : Nat)
    : Equal Bool (lt_nat a b) True → Equal Bool (leq_nat b a) True → Bottom =
  λstrict.
    λreverse.
      successor_not_self_leq
        a
        ((proof trans for leq_nat)
          (Suc a)
          b
          a
          ((proof to_leq_suc for lt_nat) a b strict)
          reverse)

fn before_step
      (q : Type)
      (d : DecEq q)
      (x : q)
      (y : q)
      (head : q)
      (tail : List q)
      (xb : Bool)
      (yb : Bool)
    : Bool =
  match xb {
    True ↦
      match yb {
        True ↦ False;
        False ↦ contains q d y tail
      };
    False ↦
      match yb {
        True ↦ False;
        False ↦ before q d x y tail
      }
  }

theorem before_cons
      (q : Type) (d : DecEq q) (x : q) (y : q) (head : q) (tail : List q)
    : Equal Bool
        (before q d x y (Cons q head tail))
        (before_step q d x y head tail (d.eq x head) (d.eq y head)) =
  Refl

theorem member_tail_if_neq
      (q : Type) (d : DecEq q) (x : q) (head : q) (tail : List q)
    : Equal Bool (d.eq x head) False → list_elem q x (Cons q head tail) → list_elem q x tail =
  λdifferent.
    λmember.
      elim_trunc
        (list_elem q x tail)
        (λcase.
          match case {
            Inl same ↦
              absurd
                (trans
                  Bool
                  False
                  (d.eq x head)
                  True
                  (sym Bool (d.eq x head) False different)
                  (d.complete x head same));
            Inr later ↦ later
          })
        member

theorem sorted_tail
      (q : Type) (le : q → q → Bool) (head : q) (rest : List q)
    : is_sorted q le (Cons q head rest) → is_sorted q le rest =
  match rest {
    Nil ↦ λsorted. Proved;
    Cons next tail ↦
      λsorted.
        and_snd (Equal Bool (le head next) True) (is_sorted q le (Cons q next tail)) sorted
  }

theorem strict_equal_absurd
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (x : q) (y : q)
    : Equal q x y → Equal Bool (lt_nat (rank q fq d g x) (rank q fq d g y)) True → Bottom =
  λsame.
    λstrict.
      let
        rx = rank q fq d g x;
        ry = rank q fq d g y;
        self_strict =
          trans
            Bool
            (lt_nat ry ry)
            (lt_nat rx ry)
            True
            (sym
              Bool
              (lt_nat rx ry)
              (lt_nat ry ry)
              (cong Nat Bool rx ry (λr. lt_nat r ry) (cong q Nat x y (rank q fq d g) same)))
            strict
      in
        strict_excludes_reverse ry ry self_strict ((proof refl for leq_nat) ry)

fn eq_field (q : Type) (d : DecEq q) (x : q) (y : q) : Bool = d.eq x y

theorem sorted_strict_before
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (xs : List q) (x : q) (y : q)
    : is_sorted q (rank_leq q fq d g) xs
      → list_elem q x xs
      → list_elem q y xs
      → Equal Bool (lt_nat (rank q fq d g x) (rank q fq d g y)) True
      → Equal Bool (before q d x y xs) True =
  match xs {
    Nil ↦ λsorted. λmember_x. λmember_y. λstrict. absurd member_x;
    Cons head rest ↦
      λsorted.
        λmember_x.
          λmember_y.
            λstrict.
              trans
                Bool
                (before q d x y (Cons q head rest))
                (before_step q d x y head rest (eq_field q d x head) (eq_field q d y head))
                True
                (before_cons q d x y head rest)
                (sorted_strict_before_at
                  q
                  fq
                  d
                  g
                  head
                  rest
                  x
                  y
                  sorted
                  member_x
                  member_y
                  strict
                  (eq_field q d x head)
                  (eq_field q d y head)
                  Refl
                  Refl)
  }

theorem sorted_strict_before_at
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (head : q)
      (rest : List q)
      (x : q)
      (y : q)
      (sorted : is_sorted q (rank_leq q fq d g) (Cons q head rest))
      (member_x : list_elem q x (Cons q head rest))
      (member_y : list_elem q y (Cons q head rest))
      (strict : Equal Bool (lt_nat (rank q fq d g x) (rank q fq d g y)) True)
      (xb : Bool)
      (yb : Bool)
    : Equal Bool (eq_field q d x head) xb
      → Equal Bool (eq_field q d y head) yb
      → Equal Bool (before_step q d x y head rest xb yb) True =
  match xb {
    True ↦
      match yb {
        True ↦
          λhx.
            λhy.
              absurd
                (strict_equal_absurd
                  q
                  fq
                  d
                  g
                  x
                  y
                  (trans q x head y (d.sound x head hx) (sym q y head (d.sound y head hy)))
                  strict);
        False ↦
          λhx.
            λhy. contains_complete q d y rest (member_tail_if_neq q d y head rest hy member_y)
      };
    False ↦
      match yb {
        True ↦
          λhx.
            λhy.
              absurd
                (strict_excludes_reverse
                  (rank q fq d g x)
                  (rank q fq d g y)
                  strict
                  (trans
                    Bool
                    (rank_leq q fq d g y x)
                    (rank_leq q fq d g head x)
                    True
                    (cong q Bool y head (λv. rank_leq q fq d g v x) (d.sound y head hy))
                    (sorted_head_le_member
                      q
                      fq
                      d
                      g
                      head
                      rest
                      x
                      sorted
                      (member_tail_if_neq q d x head rest hx member_x))));
        False ↦
          λhx.
            λhy.
              sorted_strict_before
                q
                fq
                d
                g
                rest
                x
                y
                (sorted_tail q (rank_leq q fq d g) head rest sorted)
                (member_tail_if_neq q d x head rest hx member_x)
                (member_tail_if_neq q d y head rest hy member_y)
                strict
      }
  }

theorem ordered_list_forward
      (q : Type)
      (fq : Finite q)
      (d : DecEq q)
      (g : Graph q)
      (no_cycles : (v : q) → (u : q) → no_cycle_at q fq d g v u)
    : all_edges_forward q d g (ordered_list q fq d g) =
  λx.
    λy.
      λedge_xy.
        sorted_strict_before
          q
          fq
          d
          g
          (ordered_list q fq d g)
          x
          y
          (ordered_list_sorted q fq d g)
          (contains_sound q d x (ordered_list q fq d g) (ordered_list_covers q fq d g x))
          (contains_sound q d y (ordered_list q fq d g) (ordered_list_covers q fq d g y))
          (edge_rank_strict q fq d g no_cycles x y edge_xy)
```

## 5. Design notes

The graph does not carry a `Finite` certificate: the same adjacency may be
queried with different valid enumerations. No `DecEq` is required for `Graph`,
walks, cycles, or predicate reachability; it enters only when order predicates
compare and deduplicate vertices. The result is a tagged `Type` sum because
an order or cycle carries computationally relevant evidence, unlike an
`Omega`-sorted disjunction. Neither unrelated-vertex tie order nor a shortest
walk or simple cycle is prescribed. The finite pair scan considers potential
cycle edges; it does not duplicate reachability's fixed point.

## 6. References

- [Topological sorting](https://en.wikipedia.org/wiki/Topological_sorting) —
  Wikipedia; orientation to dependency order and cycle obstruction.
- [Reachability](https://en.wikipedia.org/wiki/Reachability) — Wikipedia;
  orientation to directed path existence.
- *Introduction to Algorithms* — Cormen, Leiserson, Rivest, and Stein;
  textbook context for graph walks and dependency order.

## 7. Trust & derivation

The behavioral contract is [graphs §1](../../../../spec/50-stdlib/62-graphs.md)
and its [seed](../../../../conformance/stdlib/graphs/seed-dependency.md).
The public API is the graph, path and certificate types, their constructors,
the predicate-reachability decision and witness, the dependency-order
operation, five list/order predicates, and seven kernel-checked laws. Every
other helper is private to this package.

| Reader task | Section |
|---|---|
| Find a graph path or decision | [Definition](#2-definition), [Using it](#3-using-it) |
| Review order or cycle evidence | [Definition](#2-definition), [Laws & proofs](#4-laws--proofs) |
| Inspect search, proof and trust | [Laws & proofs](#4-laws--proofs), [Trust & derivation](#7-trust--derivation) |

The derivation uses the existing automata `reachable`/`find_word` and their
soundness/completeness laws, `Finite` coverage, and `Derived.sort` and its
sorted/permutation laws. The local graph encoding and all search and ordering
helpers are checked terms. The package adds no `Axiom`, primitive, foreign
binding, postulate or `trusted_base()` entry; this does not erase inherited
assumptions of imported providers. The graph seed and the targeted acceptance
suite exercise the ten behavior cases, generic law clients, exact public
ownership and the package-local trust delta.
