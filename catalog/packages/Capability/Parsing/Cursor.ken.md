# Capability.Parsing.Cursor

`Capability.Parsing.Cursor` is the carrier-neutral parsing floor. It exposes an
explicit operations dictionary, plain validity predicates, and a byte-structural
argument cursor.

## 1. Definition

`CursorOps` keeps element and location types explicit. Argument bytes remain
plain `Bytes`; lengths and elements come from the total `List UInt8` view.

`cursor_locate` reports the current cursor position in the instance's own
location type and coordinate system. `CursorOps` and `CursorLaws` impose no
cross-instance orientation, monotonicity, or relation between `cursor_locate`
and `cursor_remaining`. Location values are comparable only under the semantics
of the same cursor instance. Package-local conversions such as
`arg_location_origin` and `span_origin` make a diagnostic bridge explicit
without imposing a coordinate system on generic `CursorOps`.

```ken
import Capability.Diagnostics.Core
  (ArgumentOrigin,
    MkByteRange,
    Origin,
    origin_argument_index,
    origin_range_end,
    origin_range_start)

import Data.Collections.Derived (bytes_nat_length, length, nth)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (leq_nat, sub)

import Core.Logic.Transport (cong, sym, trans)

data CursorOps c el loc = MkCursorOps (c → Nat) (c → Option el) (c → c) (c → loc)

export CursorOps, MkCursorOps

pub fn cursor_remaining
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) (cur : c)
    : Nat =
  match ops {
    MkCursorOps remaining peek advance locate ↦ remaining cur
  }

pub fn cursor_peek
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) (cur : c)
    : Option el =
  match ops {
    MkCursorOps remaining peek advance locate ↦ peek cur
  }

pub fn cursor_advance
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) (cur : c)
    : c =
  match ops {
    MkCursorOps remaining peek advance locate ↦ advance cur
  }

pub fn cursor_locate
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) (cur : c)
    : loc =
  match ops {
    MkCursorOps remaining peek advance locate ↦ locate cur
  }

pub fn arg_length (arg : Bytes) : Nat = bytes_nat_length arg

data ArgLocation = MkArgLocation Nat Nat Nat

export ArgLocation, MkArgLocation

fn arg_location_index (loc : ArgLocation) : Nat =
  match loc {
    MkArgLocation index start end ↦ index
  }

fn arg_location_start (loc : ArgLocation) : Nat =
  match loc {
    MkArgLocation index start end ↦ start
  }

fn arg_location_end (loc : ArgLocation) : Nat =
  match loc {
    MkArgLocation index start end ↦ end
  }

fn arg_location_origin (loc : ArgLocation) : Origin =
  match loc {
    MkArgLocation index start end ↦ ArgumentOrigin index (MkByteRange start end)
  }

theorem arg_location_origin_index_faithful
      (index : Nat) (start : Nat) (end : Nat)
    : Equal
        (Option Nat)
        (origin_argument_index (arg_location_origin (MkArgLocation index start end)))
        (Some Nat index) =
  Refl

theorem arg_location_origin_start_faithful
      (index : Nat) (start : Nat) (end : Nat)
    : Equal
        (Option Nat)
        (origin_range_start (arg_location_origin (MkArgLocation index start end)))
        (Some Nat start) =
  Refl

theorem arg_location_origin_end_faithful
      (index : Nat) (start : Nat) (end : Nat)
    : Equal
        (Option Nat)
        (origin_range_end (arg_location_origin (MkArgLocation index start end)))
        (Some Nat end) =
  Refl

data ArgCursor = MkArgCursor (List Bytes) Nat Nat

export ArgCursor

fn arg_cursor_args (cur : ArgCursor) : List Bytes =
  match cur {
    MkArgCursor args index offset ↦ args
  }

fn arg_cursor_index (cur : ArgCursor) : Nat =
  match cur {
    MkArgCursor args index offset ↦ index
  }

fn arg_cursor_offset (cur : ArgCursor) : Nat =
  match cur {
    MkArgCursor args index offset ↦ offset
  }

pub fn cursor_nat_lt (a : Nat) (b : Nat) : Bool =
  match b {
    Zero ↦ False;
    Suc b2 ↦
      match a {
        Zero ↦ True;
        Suc a2 ↦ cursor_nat_lt a2 b2
      }
  }

fn arg_lengths_sum (args : List Bytes) : Nat =
  match args {
    Nil ↦ Zero;
    Cons arg rest ↦ add (arg_length arg) (arg_lengths_sum rest)
  }

fn arg_remaining_from (args : List Bytes) (index : Nat) (offset : Nat) : Nat =
  match index {
    Zero ↦
      match args {
        Nil ↦ Zero;
        Cons arg rest ↦ add (sub (arg_length arg) offset) (arg_lengths_sum rest)
      };
    Suc index2 ↦
      match args {
        Nil ↦ Zero;
        Cons arg rest ↦ arg_remaining_from rest index2 offset
      }
  }

fn arg_cursor_remaining (cur : ArgCursor) : Nat =
  arg_remaining_from (arg_cursor_args cur) (arg_cursor_index cur) (arg_cursor_offset cur)

fn arg_cursor_peek (cur : ArgCursor) : Option UInt8 =
  match nth Bytes (arg_cursor_index cur) (arg_cursor_args cur) {
    None ↦ None UInt8;
    Some arg ↦ nth UInt8 (arg_cursor_offset cur) (bytes_to_list arg)
  }

fn arg_cursor_normalize
      (fuel : Nat) (args : List Bytes) (index : Nat) (offset : Nat)
    : ArgCursor =
  match fuel {
    Zero ↦ MkArgCursor args index offset;
    Suc fuel2 ↦
      match nth Bytes index args {
        None ↦ MkArgCursor args index offset;
        Some arg ↦
          match cursor_nat_lt offset (arg_length arg) {
            True ↦ MkArgCursor args index offset;
            False ↦ arg_cursor_normalize fuel2 args (Suc index) Zero
          }
      }
  }

pub fn arg_cursor_start (args : List Bytes) : ArgCursor =
  arg_cursor_normalize (length Bytes args) args Zero Zero

fn arg_cursor_advance (cur : ArgCursor) : ArgCursor =
  arg_cursor_normalize
    (length Bytes (arg_cursor_args cur))
    (arg_cursor_args cur)
    (arg_cursor_index cur)
    (Suc (arg_cursor_offset cur))

fn arg_cursor_locate (cur : ArgCursor) : ArgLocation =
  MkArgLocation (arg_cursor_index cur) (arg_cursor_offset cur) (arg_cursor_offset cur)

pub const arg_cursor_ops : CursorOps ArgCursor UInt8 ArgLocation =
  MkCursorOps
    ArgCursor
    UInt8
    ArgLocation
    arg_cursor_remaining
    arg_cursor_peek
    arg_cursor_advance
    arg_cursor_locate
```

## 2. Laws

The laws are predicates over an explicit dictionary. A successful peek must
have positive remaining input, advancing such a cursor must strictly reduce
that computed bound, and a zero remaining count must be an end position.

`arg_cursor_laws` proves all three for the shipped byte cursor. Its private
bridge identifies strict `cursor_nat_lt a b` with the canonical
`leq_nat (Suc a) b` evidence used by the imported subtraction laws. The
normalization proof preserves the computed remaining count while it skips
exhausted or empty arguments; the progress proof then reduces to the strict
subtraction step at the currently selected byte.

```ken
pub fn CursorPeekHasRemaining
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc)
    : Prop =
  (cur : c)
    → (value : el)
    → Equal
    (Option el)
    (cursor_peek c el loc ops cur)
    (Some el value)
    → Equal Bool
    (cursor_nat_lt Zero (cursor_remaining c el loc ops cur))
    True

pub fn CursorAdvanceProgress
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc)
    : Prop =
  (cur : c)
    → (value : el)
    → Equal
    (Option el)
    (cursor_peek c el loc ops cur)
    (Some el value)
    → Equal Bool
    (cursor_nat_lt
      (cursor_remaining c el loc ops (cursor_advance c el loc ops cur))
      (cursor_remaining c el loc ops cur))
    True

pub fn CursorEndValid (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) : Prop =
  (cur : c)
    → Equal Nat
    (cursor_remaining c el loc ops cur)
    Zero → Equal
    (Option el)
    (cursor_peek c el loc ops cur)
    (None el)

pub fn CursorLaws (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) : Prop =
  And
    (CursorPeekHasRemaining c el loc ops)
    (And (CursorAdvanceProgress c el loc ops) (CursorEndValid c el loc ops))

theorem cursor_nat_lt_from_leq_suc
      (a : Nat)
    : (b : Nat) → IsTrue (leq_nat (Suc a) b) → Equal Bool (cursor_nat_lt a b) True =
  λb.
    match b {
      Zero ↦ λh. absurd h;
      Suc b2 ↦
        match a {
          Zero ↦ λh. Proved;
          Suc a2 ↦ λh. cursor_nat_lt_from_leq_suc a2 b2 h
        }
    }

theorem cursor_nat_lt_to_leq_suc
      (a : Nat)
    : (b : Nat) → Equal Bool (cursor_nat_lt a b) True → IsTrue (leq_nat (Suc a) b) =
  λb.
    match b {
      Zero ↦ λh. absurd h;
      Suc b2 ↦
        match a {
          Zero ↦ λh. Proved;
          Suc a2 ↦ λh. cursor_nat_lt_to_leq_suc a2 b2 h
        }
    }

theorem cursor_nat_not_lt_to_reverse_leq
      (a : Nat)
    : (b : Nat) → Equal Bool (cursor_nat_lt a b) False → IsTrue (leq_nat b a) =
  λb.
    match b {
      Zero ↦ λh. Proved;
      Suc b2 ↦
        match a {
          Zero ↦ λh. absurd h;
          Suc a2 ↦ λh. cursor_nat_not_lt_to_reverse_leq a2 b2 h
        }
    }

theorem cursor_nat_lt_zero_add_suc
      (n : Nat) (rest : Nat)
    : Equal Bool (cursor_nat_lt Zero (add (Suc n) rest)) True =
  match rest {
    Zero ↦ Proved;
    Suc rest2 ↦ Proved
  }

theorem cursor_nat_lt_zero_from_leq_suc
      (witness : Nat)
    : (larger : Nat)
      → IsTrue (leq_nat (Suc witness) larger)
      → Equal Bool (cursor_nat_lt Zero larger) True =
  λlarger.
    match larger {
      Zero ↦ λordered. absurd ordered;
      Suc larger2 ↦ λordered. Proved
    }

theorem cursor_nat_lt_zero_add_from_positive
      (n : Nat) (rest : Nat)
    : Equal Bool (cursor_nat_lt Zero n) True
      → Equal Bool (cursor_nat_lt Zero (add n rest)) True =
  match n {
    Zero ↦ λpositive. absurd positive;
    Suc n2 ↦ λpositive. cursor_nat_lt_zero_add_suc n2 rest
  }

theorem cursor_leq_suc_add_right
      (smaller : Nat) (larger : Nat) (rest : Nat)
    : IsTrue (leq_nat (Suc smaller) larger)
      → IsTrue (leq_nat (Suc (add smaller rest)) (add larger rest)) =
  match rest {
    Zero ↦ λh. h;
    Suc rest2 ↦ λh. cursor_leq_suc_add_right smaller larger rest2 h
  }

theorem arg_remaining_shift_past_end
      (args : List Bytes)
    : (index : Nat)
      → (offset : Nat)
      → (arg : Bytes)
      → Equal (Option Bytes) (nth Bytes index args) (Some Bytes arg)
      → Equal Bool (cursor_nat_lt offset (arg_length arg)) False
      → Equal Nat
        (arg_remaining_from args index offset)
        (arg_remaining_from args (Suc index) Zero) =
  match args {
    Nil ↦ λindex. λoffset. λarg. λfound. λpast. absurd found;
    Cons head tail ↦
      λindex.
        match index {
          Zero ↦
            λoffset.
              λarg.
                λfound.
                  let same_arg : Equal Bytes head arg =
                    cong
                      (Option Bytes)
                      Bytes
                      (Some Bytes head)
                      (Some Bytes arg)
                      (λselected.
                        match selected {
                          None ↦ head;
                          Some present ↦ present
                        })
                      found
                  in
                    J
                      (λarg' _.
                        Equal Bool (cursor_nat_lt offset (arg_length arg')) False
                        → Equal
                          Nat
                          (arg_remaining_from (Cons Bytes head tail) Zero offset)
                          (arg_remaining_from (Cons Bytes head tail) (Suc Zero) Zero))
                      (λpast.
                        trans
                          Nat
                          (add (sub (arg_length head) offset) (arg_lengths_sum tail))
                          (add Zero (arg_lengths_sum tail))
                          (arg_remaining_from (Cons Bytes head tail) (Suc Zero) Zero)
                          (cong
                            Nat
                            Nat
                            (sub (arg_length head) offset)
                            Zero
                            (λremaining. add remaining (arg_lengths_sum tail))
                            ((proof saturates for sub)
                              (arg_length head)
                              offset
                              (cursor_nat_not_lt_to_reverse_leq offset (arg_length head) past)))
                          ((proof zero_l for add) (arg_lengths_sum tail)))
                      same_arg;
          Suc index2 ↦
            λoffset.
              λarg.
                λfound. λpast. arg_remaining_shift_past_end tail index2 offset arg found past
        }
  }

fn arg_cursor_normalize_step
      (fuel : Nat) (args : List Bytes) (index : Nat) (offset : Nat) (selected : Option Bytes)
    : ArgCursor =
  match selected {
    None ↦ MkArgCursor args index offset;
    Some arg ↦
      match cursor_nat_lt offset (arg_length arg) {
        True ↦ MkArgCursor args index offset;
        False ↦ arg_cursor_normalize fuel args (Suc index) Zero
      }
  }

theorem arg_cursor_normalize_suc_unfold
      (fuel : Nat) (args : List Bytes) (index : Nat) (offset : Nat)
    : Equal ArgCursor
        (arg_cursor_normalize (Suc fuel) args index offset)
        (arg_cursor_normalize_step fuel args index offset (nth Bytes index args)) =
  Refl

theorem arg_cursor_normalize_step_in_bounds
      (fuel : Nat) (args : List Bytes) (index : Nat) (offset : Nat) (arg : Bytes)
    : Equal Bool (cursor_nat_lt offset (arg_length arg)) True
      → Equal ArgCursor
        (arg_cursor_normalize_step fuel args index offset (Some Bytes arg))
        (MkArgCursor args index offset) =
  λin_bounds.
    cong
      Bool
      ArgCursor
      (cursor_nat_lt offset (arg_length arg))
      True
      (λcomparison.
        match comparison {
          True ↦ MkArgCursor args index offset;
          False ↦ arg_cursor_normalize fuel args (Suc index) Zero
        })
      in_bounds

theorem arg_cursor_normalize_step_past_end
      (fuel : Nat) (args : List Bytes) (index : Nat) (offset : Nat) (arg : Bytes)
    : Equal Bool (cursor_nat_lt offset (arg_length arg)) False
      → Equal ArgCursor
        (arg_cursor_normalize_step fuel args index offset (Some Bytes arg))
        (arg_cursor_normalize fuel args (Suc index) Zero) =
  λpast_end.
    cong
      Bool
      ArgCursor
      (cursor_nat_lt offset (arg_length arg))
      False
      (λcomparison.
        match comparison {
          True ↦ MkArgCursor args index offset;
          False ↦ arg_cursor_normalize fuel args (Suc index) Zero
        })
      past_end

theorem arg_cursor_normalize_some_preserves
      (comparison : Bool)
    : (fuel : Nat)
      → (args : List Bytes)
      → (index : Nat)
      → (offset : Nat)
      → (arg : Bytes)
      → Equal Bool comparison (cursor_nat_lt offset (arg_length arg))
      → Equal (Option Bytes) (Some Bytes arg) (nth Bytes index args)
      → Equal Nat
        (arg_cursor_remaining (arg_cursor_normalize fuel args (Suc index) Zero))
        (arg_remaining_from args (Suc index) Zero)
      → Equal Nat
        (arg_cursor_remaining
          (arg_cursor_normalize_step fuel args index offset (Some Bytes arg)))
        (arg_remaining_from args index offset) =
  match comparison {
    True ↦
      λfuel.
        λargs.
          λindex.
            λoffset.
              λarg.
                λcomparison_is_cursor.
                  λselected_is_nth.
                    λrecursive.
                      trans
                        Nat
                        (arg_cursor_remaining
                          (arg_cursor_normalize_step fuel args index offset (Some Bytes arg)))
                        (arg_cursor_remaining (MkArgCursor args index offset))
                        (arg_remaining_from args index offset)
                        (cong
                          ArgCursor
                          Nat
                          (arg_cursor_normalize_step fuel args index offset (Some Bytes arg))
                          (MkArgCursor args index offset)
                          arg_cursor_remaining
                          (arg_cursor_normalize_step_in_bounds
                            fuel
                            args
                            index
                            offset
                            arg
                            (sym
                              Bool
                              True
                              (cursor_nat_lt offset (arg_length arg))
                              comparison_is_cursor)))
                        Refl;
    False ↦
      λfuel.
        λargs.
          λindex.
            λoffset.
              λarg.
                λcomparison_is_cursor.
                  λselected_is_nth.
                    λrecursive.
                      trans
                        Nat
                        (arg_cursor_remaining
                          (arg_cursor_normalize_step fuel args index offset (Some Bytes arg)))
                        (arg_cursor_remaining (arg_cursor_normalize fuel args (Suc index) Zero))
                        (arg_remaining_from args index offset)
                        (cong
                          ArgCursor
                          Nat
                          (arg_cursor_normalize_step fuel args index offset (Some Bytes arg))
                          (arg_cursor_normalize fuel args (Suc index) Zero)
                          arg_cursor_remaining
                          (arg_cursor_normalize_step_past_end
                            fuel
                            args
                            index
                            offset
                            arg
                            (sym
                              Bool
                              False
                              (cursor_nat_lt offset (arg_length arg))
                              comparison_is_cursor)))
                        (trans
                          Nat
                          (arg_cursor_remaining
                            (arg_cursor_normalize fuel args (Suc index) Zero))
                          (arg_remaining_from args (Suc index) Zero)
                          (arg_remaining_from args index offset)
                          recursive
                          (sym
                            Nat
                            (arg_remaining_from args index offset)
                            (arg_remaining_from args (Suc index) Zero)
                            (arg_remaining_shift_past_end
                              args
                              index
                              offset
                              arg
                              (sym
                                (Option Bytes)
                                (Some Bytes arg)
                                (nth Bytes index args)
                                selected_is_nth)
                              (sym
                                Bool
                                False
                                (cursor_nat_lt offset (arg_length arg))
                                comparison_is_cursor))))
  }

theorem arg_cursor_normalize_selected_preserves
      (selected : Option Bytes)
    : (fuel : Nat)
      → (args : List Bytes)
      → (index : Nat)
      → (offset : Nat)
      → Equal (Option Bytes) selected (nth Bytes index args)
      → Equal Nat
        (arg_cursor_remaining (arg_cursor_normalize fuel args (Suc index) Zero))
        (arg_remaining_from args (Suc index) Zero)
      → Equal Nat
        (arg_cursor_remaining (arg_cursor_normalize_step fuel args index offset selected))
        (arg_remaining_from args index offset) =
  match selected {
    None ↦ λfuel. λargs. λindex. λoffset. λselected_is_nth. λrecursive. Refl;
    Some arg ↦
      λfuel.
        λargs.
          λindex.
            λoffset.
              λselected_is_nth.
                λrecursive.
                  arg_cursor_normalize_some_preserves
                    (cursor_nat_lt offset (arg_length arg))
                    fuel
                    args
                    index
                    offset
                    arg
                    Refl
                    selected_is_nth
                    recursive
  }

theorem arg_cursor_normalize_preserves_remaining
      (fuel : Nat) (args : List Bytes) (index : Nat) (offset : Nat)
    : Equal Nat
        (arg_cursor_remaining (arg_cursor_normalize fuel args index offset))
        (arg_remaining_from args index offset) =
  match fuel {
    Zero ↦ Refl;
    Suc fuel2 ↦
      trans
        Nat
        (arg_cursor_remaining (arg_cursor_normalize (Suc fuel2) args index offset))
        (arg_cursor_remaining
          (arg_cursor_normalize_step fuel2 args index offset (nth Bytes index args)))
        (arg_remaining_from args index offset)
        (cong
          ArgCursor
          Nat
          (arg_cursor_normalize (Suc fuel2) args index offset)
          (arg_cursor_normalize_step fuel2 args index offset (nth Bytes index args))
          arg_cursor_remaining
          (arg_cursor_normalize_suc_unfold fuel2 args index offset))
        (arg_cursor_normalize_selected_preserves
          (nth Bytes index args)
          fuel2
          args
          index
          offset
          Refl
          (arg_cursor_normalize_preserves_remaining fuel2 args (Suc index) Zero))
  }

theorem cursor_nat_lt_zero_contradicts_empty
      (n : Nat)
    : Equal Bool (cursor_nat_lt Zero n) True → Equal Nat n Zero → Bottom =
  λpositive.
    λempty.
      absurd
        (trans
          Bool
          True
          (cursor_nat_lt Zero n)
          False
          (sym Bool (cursor_nat_lt Zero n) True positive)
          (cong Nat Bool n Zero (cursor_nat_lt Zero) empty))

theorem arg_cursor_peek_has_remaining_from
      (args : List Bytes)
    : (index : Nat)
      → (offset : Nat)
      → (value : UInt8)
      → Equal
        (Option UInt8)
        (arg_cursor_peek (MkArgCursor args index offset))
        (Some UInt8 value)
      → Equal Bool (cursor_nat_lt Zero (arg_remaining_from args index offset)) True =
  match args {
    Nil ↦ λindex. λoffset. λvalue. λpeeked. absurd peeked;
    Cons arg rest ↦
      λindex.
        match index {
          Zero ↦
            λoffset.
              λvalue.
                λpeeked.
                  let
                    in_bounds : IsTrue (leq_nat (Suc offset) (arg_length arg)) =
                      (proof some_below_length for nth)
                        UInt8
                        offset
                        (bytes_to_list arg)
                        value
                        peeked;
                    decreases : IsTrue
                      (leq_nat
                        (Suc (sub (arg_length arg) (Suc offset)))
                        (sub (arg_length arg) offset)) =
                      (proof suc_decreases for sub) (arg_length arg) offset in_bounds;
                    positive_sub : Equal Bool
                      (cursor_nat_lt Zero (sub (arg_length arg) offset))
                      True =
                      cursor_nat_lt_zero_from_leq_suc
                        (sub (arg_length arg) (Suc offset))
                        (sub (arg_length arg) offset)
                        decreases
                  in
                    cursor_nat_lt_zero_add_from_positive
                      (sub (arg_length arg) offset)
                      (arg_lengths_sum rest)
                      positive_sub;
          Suc index2 ↦
            λoffset.
              λvalue.
                λpeeked. arg_cursor_peek_has_remaining_from rest index2 offset value peeked
        }
  }

theorem arg_remaining_advance_decreases_from_peek
      (args : List Bytes)
    : (index : Nat)
      → (offset : Nat)
      → (value : UInt8)
      → Equal
        (Option UInt8)
        (arg_cursor_peek (MkArgCursor args index offset))
        (Some UInt8 value)
      → Equal Bool
        (cursor_nat_lt
          (arg_remaining_from args index (Suc offset))
          (arg_remaining_from args index offset))
        True =
  match args {
    Nil ↦ λindex. λoffset. λvalue. λpeeked. absurd peeked;
    Cons arg rest ↦
      λindex.
        match index {
          Zero ↦
            λoffset.
              λvalue.
                λpeeked.
                  let
                    in_bounds : IsTrue (leq_nat (Suc offset) (arg_length arg)) =
                      (proof some_below_length for nth)
                        UInt8
                        offset
                        (bytes_to_list arg)
                        value
                        peeked;
                    decreases : IsTrue
                      (leq_nat
                        (Suc (sub (arg_length arg) (Suc offset)))
                        (sub (arg_length arg) offset)) =
                      (proof suc_decreases for sub) (arg_length arg) offset in_bounds;
                    lifted : IsTrue
                      (leq_nat
                        (Suc (add (sub (arg_length arg) (Suc offset)) (arg_lengths_sum rest)))
                        (add (sub (arg_length arg) offset) (arg_lengths_sum rest))) =
                      cursor_leq_suc_add_right
                        (sub (arg_length arg) (Suc offset))
                        (sub (arg_length arg) offset)
                        (arg_lengths_sum rest)
                        decreases
                  in
                    cursor_nat_lt_from_leq_suc
                      (add (sub (arg_length arg) (Suc offset)) (arg_lengths_sum rest))
                      (add (sub (arg_length arg) offset) (arg_lengths_sum rest))
                      lifted;
          Suc index2 ↦
            λoffset.
              λvalue.
                λpeeked.
                  arg_remaining_advance_decreases_from_peek rest index2 offset value peeked
        }
  }

theorem arg_cursor_end_valid_at_current
      (comparison : Bool)
    : (arg : Bytes)
      → (rest : List Bytes)
      → (offset : Nat)
      → Equal Bool comparison (cursor_nat_lt offset (arg_length arg))
      → Equal Nat (add (sub (arg_length arg) offset) (arg_lengths_sum rest)) Zero
      → Equal (Option UInt8) (nth UInt8 offset (bytes_to_list arg)) (None UInt8) =
  match comparison {
    True ↦
      λarg.
        λrest.
          λoffset.
            λcomparison_is_cursor.
              λempty.
                let
                  in_bounds : IsTrue (leq_nat (Suc offset) (arg_length arg)) =
                    cursor_nat_lt_to_leq_suc
                      offset
                      (arg_length arg)
                      (sym
                        Bool
                        True
                        (cursor_nat_lt offset (arg_length arg))
                        comparison_is_cursor);
                  decreases : IsTrue
                    (leq_nat
                      (Suc (sub (arg_length arg) (Suc offset)))
                      (sub (arg_length arg) offset)) =
                    (proof suc_decreases for sub) (arg_length arg) offset in_bounds;
                  positive_sub : Equal Bool
                    (cursor_nat_lt Zero (sub (arg_length arg) offset))
                    True =
                    cursor_nat_lt_zero_from_leq_suc
                      (sub (arg_length arg) (Suc offset))
                      (sub (arg_length arg) offset)
                      decreases;
                  positive_remaining : Equal Bool
                    (cursor_nat_lt
                      Zero
                      (add (sub (arg_length arg) offset) (arg_lengths_sum rest)))
                    True =
                    cursor_nat_lt_zero_add_from_positive
                      (sub (arg_length arg) offset)
                      (arg_lengths_sum rest)
                      positive_sub
                in
                  absurd
                    (cursor_nat_lt_zero_contradicts_empty
                      (add (sub (arg_length arg) offset) (arg_lengths_sum rest))
                      positive_remaining
                      empty);
    False ↦
      λarg.
        λrest.
          λoffset.
            λcomparison_is_cursor.
              λempty.
                (proof at_or_beyond_is_none for nth)
                  UInt8
                  offset
                  (bytes_to_list arg)
                  (cursor_nat_not_lt_to_reverse_leq
                    offset
                    (arg_length arg)
                    (sym
                      Bool
                      False
                      (cursor_nat_lt offset (arg_length arg))
                      comparison_is_cursor))
  }

theorem arg_cursor_end_valid_from
      (args : List Bytes)
    : (index : Nat)
      → (offset : Nat)
      → Equal Nat (arg_remaining_from args index offset) Zero
      → Equal (Option UInt8) (arg_cursor_peek (MkArgCursor args index offset)) (None UInt8) =
  match args {
    Nil ↦ λindex. λoffset. λempty. Proved;
    Cons arg rest ↦
      λindex.
        match index {
          Zero ↦
            λoffset.
              arg_cursor_end_valid_at_current
                (cursor_nat_lt offset (arg_length arg))
                arg
                rest
                offset
                Refl;
          Suc index2 ↦ λoffset. λempty. arg_cursor_end_valid_from rest index2 offset empty
        }
  }

theorem arg_cursor_peek_has_remaining
    : CursorPeekHasRemaining ArgCursor UInt8 ArgLocation arg_cursor_ops =
  λcur.
    match cur {
      MkArgCursor args index offset ↦ arg_cursor_peek_has_remaining_from args index offset
    }

theorem arg_cursor_advance_progress
    : CursorAdvanceProgress ArgCursor UInt8 ArgLocation arg_cursor_ops =
  λcur.
    match cur {
      MkArgCursor args index offset ↦
        λvalue.
          λpeeked.
            let
              raw_progress : Equal Bool
                (cursor_nat_lt
                  (arg_remaining_from args index (Suc offset))
                  (arg_remaining_from args index offset))
                True =
                arg_remaining_advance_decreases_from_peek args index offset value peeked;
              normalized_remaining : Equal Nat
                (arg_cursor_remaining
                  (arg_cursor_normalize (length Bytes args) args index (Suc offset)))
                (arg_remaining_from args index (Suc offset)) =
                arg_cursor_normalize_preserves_remaining
                  (length Bytes args)
                  args
                  index
                  (Suc offset)
            in
              J
                (λadvanced _.
                  Equal
                    Bool
                    (cursor_nat_lt advanced (arg_remaining_from args index offset))
                    True)
                raw_progress
                (sym
                  Nat
                  (arg_cursor_remaining
                    (arg_cursor_normalize (length Bytes args) args index (Suc offset)))
                  (arg_remaining_from args index (Suc offset))
                  normalized_remaining)
    }

theorem arg_cursor_end_valid : CursorEndValid ArgCursor UInt8 ArgLocation arg_cursor_ops =
  λcur.
    match cur {
      MkArgCursor args index offset ↦ arg_cursor_end_valid_from args index offset
    }

pub theorem arg_cursor_laws : CursorLaws ArgCursor UInt8 ArgLocation arg_cursor_ops =
  and_intro
    (CursorPeekHasRemaining ArgCursor UInt8 ArgLocation arg_cursor_ops)
    (And
      (CursorAdvanceProgress ArgCursor UInt8 ArgLocation arg_cursor_ops)
      (CursorEndValid ArgCursor UInt8 ArgLocation arg_cursor_ops))
    arg_cursor_peek_has_remaining
    (and_intro
      (CursorAdvanceProgress ArgCursor UInt8 ArgLocation arg_cursor_ops)
      (CursorEndValid ArgCursor UInt8 ArgLocation arg_cursor_ops)
      arg_cursor_advance_progress
      arg_cursor_end_valid)
```

## 3. Using it

Pass raw argument bytes directly to `arg_cursor_start`. It normalizes empty
arguments; ordinary `cursor_advance` then crosses argument boundaries while
preserving exact argument and byte positions.

## 4. Design notes

Lengths and elements are computed from the structural byte view. Repetition
fuel is derived from `arg_cursor_remaining`; no caller-supplied length or proof
is accepted.

## 5. References

None.

## 6. Trust  derivation

All declarations are transparent checked terms over landed `Bytes`, `List`,
`Option`, and equality. The exported cursor-law inhabitant and its private
bridges are kernel-checked structural proofs. This package adds no axiom,
primitive, or postulate.

## 7. Package  summary

Public surface: `CursorOps`, its four selectors and laws, `arg_length`,
`ArgLocation`, `ArgCursor`, `arg_cursor_start`, `arg_cursor_ops`, and the
checked `arg_cursor_laws` inhabitant.
