# Capability.Parsing.Decoder

`Capability.Parsing.Decoder` supplies progress-safe parser combinators over
an explicit `CursorOps`. Its result and error carriers remain generic in the
cursor's location type, keeping downstream source and argument diagnostics
separate.

## 1. Definition

Ordinary rejection is backtrackable. Zero progress and impossible fuel
exhaustion are named, non-backtrackable failures so repetition cannot silently
loop or truncate.

decoder combinators preserve the supplied instance location opaquely; errors
from different `CursorOps` instances are not position-comparable without an
explicit conversion chosen by the caller.

```ken
import Capability.Parsing.Cursor
  (CursorOps, cursor_advance, cursor_locate, cursor_nat_lt, cursor_peek, cursor_remaining)

export DecoderError, DecoderRejected

data DecoderError loc = DecoderRejected loc | DecoderZeroProgress loc | DecoderFuelExhausted loc

export DecoderResult, Decoded, DecoderFailed

data DecoderResult c loc a = Decoded a c | DecoderFailed (DecoderError loc)

pub const Decoder (c : Type) (loc : Type) (a : Type) : Type = c → DecoderResult c loc a

pub fn decoder_error_location (loc : Type) (err : DecoderError loc) : loc =
  match err {
    DecoderRejected at ↦ at;
    DecoderZeroProgress at ↦ at;
    DecoderFuelExhausted at ↦ at
  }

pub fn decoder_pure (c : Type) (loc : Type) (a : Type) (value : a) : Decoder c loc a =
  λcur. Decoded c loc a value cur

pub fn decoder_fail
      (c : Type) (el : Type) (loc : Type) (a : Type) (ops : CursorOps c el loc)
    : Decoder c loc a =
  λcur. DecoderFailed c loc a (DecoderRejected loc (cursor_locate c el loc ops cur))

fn decoder_map
      (c : Type) (loc : Type) (a : Type) (b : Type) (f : a → b) (decoder : Decoder c loc a)
    : Decoder c loc b =
  λcur.
    match decoder cur {
      Decoded value next ↦ Decoded c loc b (f value) next;
      DecoderFailed err ↦ DecoderFailed c loc b err
    }

pub fn decoder_bind
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (decoder : Decoder c loc a)
      (next_decoder : a → Decoder c loc b)
    : Decoder c loc b =
  λcur.
    match decoder cur {
      Decoded value next ↦ next_decoder value next;
      DecoderFailed err ↦ DecoderFailed c loc b err
    }

pub fn decoder_seq
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc b)
    : Decoder c loc b =
  decoder_bind c loc a b first (λignored. second)

pub fn decoder_alt
      (c : Type) (loc : Type) (a : Type) (first : Decoder c loc a) (second : Decoder c loc a)
    : Decoder c loc a =
  λcur.
    match first cur {
      Decoded value next ↦ Decoded c loc a value next;
      DecoderFailed err ↦
        match err {
          DecoderRejected at ↦ second cur;
          DecoderZeroProgress at ↦ DecoderFailed c loc a (DecoderZeroProgress loc at);
          DecoderFuelExhausted at ↦ DecoderFailed c loc a (DecoderFuelExhausted loc at)
        }
    }

pub fn decoder_satisfy
      (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc) (accept : el → Bool)
    : Decoder c loc el =
  λcur.
    match cursor_peek c el loc ops cur {
      None ↦ DecoderFailed c loc el (DecoderRejected loc (cursor_locate c el loc ops cur));
      Some value ↦
        match accept value {
          True ↦ Decoded c loc el value (cursor_advance c el loc ops cur);
          False ↦ DecoderFailed c loc el (DecoderRejected loc (cursor_locate c el loc ops cur))
        }
    }

fn decoder_token
      (c : Type)
      (el : Type)
      (loc : Type)
      (ops : CursorOps c el loc)
      (equal : el → el → Bool)
      (expected : el)
    : Decoder c loc el =
  decoder_satisfy c el loc ops (λactual. equal actual expected)

fn decoder_many_fuel
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (fuel : Nat)
      (cur : c)
    : DecoderResult c loc (List a) =
  match fuel {
    Zero ↦
      match cursor_remaining c el loc ops cur {
        Zero ↦ Decoded c loc (List a) (Nil a) cur;
        Suc rest ↦
          DecoderFailed
            c
            loc
            (List a)
            (DecoderFuelExhausted loc (cursor_locate c el loc ops cur))
      };
    Suc fuel2 ↦
      match step cur {
        DecoderFailed err ↦
          match err {
            DecoderRejected at ↦ Decoded c loc (List a) (Nil a) cur;
            DecoderZeroProgress at ↦ DecoderFailed c loc (List a) (DecoderZeroProgress loc at);
            DecoderFuelExhausted at ↦ DecoderFailed c loc (List a) (DecoderFuelExhausted loc at)
          };
        Decoded value next ↦
          match cursor_nat_lt
            (cursor_remaining c el loc ops next)
            (cursor_remaining c el loc ops cur) {
            False ↦
              DecoderFailed
                c
                loc
                (List a)
                (DecoderZeroProgress loc (cursor_locate c el loc ops next));
            True ↦
              match decoder_many_fuel c el loc a ops step fuel2 next {
                DecoderFailed err ↦ DecoderFailed c loc (List a) err;
                Decoded rest end ↦ Decoded c loc (List a) (Cons a value rest) end
              }
          }
      }
  }

pub fn decoder_many
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
    : Decoder c loc (List a) =
  λcur. decoder_many_fuel c el loc a ops step (cursor_remaining c el loc ops cur) cur

fn decoder_some
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
    : Decoder c loc (List a) =
  λcur.
    match step cur {
      DecoderFailed err ↦ DecoderFailed c loc (List a) err;
      Decoded value next ↦
        match cursor_nat_lt
          (cursor_remaining c el loc ops next)
          (cursor_remaining c el loc ops cur) {
          False ↦
            DecoderFailed
              c
              loc
              (List a)
              (DecoderZeroProgress loc (cursor_locate c el loc ops next));
          True ↦
            match decoder_many c el loc a ops step next {
              DecoderFailed err ↦ DecoderFailed c loc (List a) err;
              Decoded rest end ↦ Decoded c loc (List a) (Cons a value rest) end
            }
        }
    }

fn decoder_recursive_fuel
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (layer : Decoder c loc a → Decoder c loc a)
      (fuel : Nat)
      (cur : c)
    : DecoderResult c loc a =
  match fuel {
    Zero ↦ DecoderFailed c loc a (DecoderFuelExhausted loc (cursor_locate c el loc ops cur));
    Suc fuel2 ↦ layer (decoder_recursive_fuel c el loc a ops layer fuel2) cur
  }

pub fn decoder_recursive
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (layer : Decoder c loc a → Decoder c loc a)
    : Decoder c loc a =
  λcur. decoder_recursive_fuel c el loc a ops layer (cursor_remaining c el loc ops cur) cur
```

## 2. Laws

`DecoderProgress` is the obligation on a repeated step. The whole-input law is
the observable contract of a successful repetition: success ends only when the
derived remaining bound is zero.

The private equation suite states each combinator directly over its existing
result representation. The equations quantify over every cursor and every
first-decoder outcome: they do not assume a normalized cursor, positive
remaining input, or eventual acceptance. The recursive equation also exposes
both the zero-fuel failure and successor layer selected by the cursor's actual
remaining value.

The repetition proof is structural on the derived fuel. At zero fuel, a
successful result transports the matched `remaining = Zero` fact to its end
cursor. At successor fuel, ordinary rejection uses
`DecoderRejectsOnlyAtEnd`, a non-decreasing success contradicts
`DecoderProgress`, and a decreasing success recurses on the smaller fuel.
Failure branches cannot inhabit the successful-result premise. No step proves
that the derived fuel is sufficient, and callers never supply a bound.

```ken
fn DecoderProgress
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
    : Prop =
  (cur : c)
    → (value : a)
    → (next : c)
    → Equal
    (DecoderResult c loc a)
    (step cur)
    (Decoded c loc a value next)
    → Equal Bool
    (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
    True

fn DecoderConsumesAll
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (decoder : Decoder c loc a)
    : Prop =
  (cur : c)
    → (value : a)
    → (end : c)
    → Equal
    (DecoderResult c loc a)
    (decoder cur)
    (Decoded c loc a value end)
    → Equal Nat
    (cursor_remaining c el loc ops end)
    Zero

fn DecoderRejectsOnlyAtEnd
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
    : Prop =
  (cur : c)
    → (at : loc)
    → Equal
    (DecoderResult c loc a)
    (step cur)
    (DecoderFailed c loc a (DecoderRejected loc at))
    → Equal Nat
    (cursor_remaining c el loc ops cur)
    Zero

fn DecoderManyConsumesAllLaw
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
    : Prop =
  And (DecoderProgress c el loc a ops step) (DecoderRejectsOnlyAtEnd c el loc a ops step)
  → DecoderConsumesAll c el loc (List a) ops (decoder_many c el loc a ops step)

fn decoder_bind_result
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (next_decoder : a → Decoder c loc b)
      (outcome : DecoderResult c loc a)
    : DecoderResult c loc b =
  match outcome {
    Decoded value next ↦ next_decoder value next;
    DecoderFailed err ↦ DecoderFailed c loc b err
  }

fn decoder_seq_next
      (c : Type) (loc : Type) (a : Type) (b : Type) (second : Decoder c loc b) (ignored : a)
    : Decoder c loc b =
  second

fn decoder_alt_result
      (c : Type)
      (loc : Type)
      (a : Type)
      (second : Decoder c loc a)
      (cur : c)
      (outcome : DecoderResult c loc a)
    : DecoderResult c loc a =
  match outcome {
    Decoded value next ↦ Decoded c loc a value next;
    DecoderFailed err ↦
      match err {
        DecoderRejected at ↦ second cur;
        DecoderZeroProgress at ↦ DecoderFailed c loc a (DecoderZeroProgress loc at);
        DecoderFuelExhausted at ↦ DecoderFailed c loc a (DecoderFuelExhausted loc at)
      }
  }

fn decoder_recursive_result
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (layer : Decoder c loc a → Decoder c loc a)
      (cur : c)
      (fuel : Nat)
    : DecoderResult c loc a =
  match fuel {
    Zero ↦ DecoderFailed c loc a (DecoderFuelExhausted loc (cursor_locate c el loc ops cur));
    Suc fuel2 ↦ layer (decoder_recursive_fuel c el loc a ops layer fuel2) cur
  }

theorem decoder_pure_equation
      (c : Type) (loc : Type) (a : Type) (value : a) (cur : c)
    : Equal
        (DecoderResult c loc a)
        (decoder_pure c loc a value cur)
        (Decoded c loc a value cur) =
  Refl

theorem decoder_fail_equation
      (c : Type) (el : Type) (loc : Type) (a : Type) (ops : CursorOps c el loc) (cur : c)
    : Equal
        (DecoderResult c loc a)
        (decoder_fail c el loc a ops cur)
        (DecoderFailed c loc a (DecoderRejected loc (cursor_locate c el loc ops cur))) =
  Refl

theorem decoder_bind_equation
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (decoder : Decoder c loc a)
      (next_decoder : a → Decoder c loc b)
      (cur : c)
    : Equal
        (DecoderResult c loc b)
        (decoder_bind c loc a b decoder next_decoder cur)
        (decoder_bind_result c loc a b next_decoder (decoder cur)) =
  Refl

theorem decoder_seq_equation
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc b)
      (cur : c)
    : Equal
        (DecoderResult c loc b)
        (decoder_seq c loc a b first second cur)
        (decoder_bind_result c loc a b (decoder_seq_next c loc a b second) (first cur)) =
  Refl

theorem decoder_alt_equation
      (c : Type)
      (loc : Type)
      (a : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc a)
      (cur : c)
    : Equal
        (DecoderResult c loc a)
        (decoder_alt c loc a first second cur)
        (decoder_alt_result c loc a second cur (first cur)) =
  Refl

theorem decoder_recursive_equation
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (layer : Decoder c loc a → Decoder c loc a)
      (cur : c)
    : Equal
        (DecoderResult c loc a)
        (decoder_recursive c el loc a ops layer cur)
        (decoder_recursive_result
          c
          el
          loc
          a
          ops
          layer
          cur
          (cursor_remaining c el loc ops cur)) =
  Refl

fn decoder_result_end_or
      (c : Type) (loc : Type) (a : Type) (fallback : c) (outcome : DecoderResult c loc a)
    : c =
  match outcome {
    Decoded value end ↦ end;
    DecoderFailed err ↦ fallback
  }

theorem decoder_error_elim
      (loc : Type)
      (motive : DecoderError loc → Prop)
      (err : DecoderError loc)
      (rejected : (at : loc) → motive (DecoderRejected loc at))
      (zero_progress : (at : loc) → motive (DecoderZeroProgress loc at))
      (fuel_exhausted : (at : loc) → motive (DecoderFuelExhausted loc at))
    : motive err =
  match err {
    DecoderRejected at ↦ rejected at;
    DecoderZeroProgress at ↦ zero_progress at;
    DecoderFuelExhausted at ↦ fuel_exhausted at
  }

theorem decoder_result_elim
      (c : Type)
      (loc : Type)
      (a : Type)
      (motive : DecoderResult c loc a → Prop)
      (outcome : DecoderResult c loc a)
      (decoded : (value : a) → (end : c) → motive (Decoded c loc a value end))
      (failed : (err : DecoderError loc) → motive (DecoderFailed c loc a err))
    : motive outcome =
  match outcome {
    Decoded value end ↦ decoded value end;
    DecoderFailed err ↦ failed err
  }

theorem decoder_success_end
      (c : Type)
      (loc : Type)
      (a : Type)
      (fallback : c)
      (outcome : DecoderResult c loc a)
      (value : a)
      (end : c)
      (succeeded : Equal (DecoderResult c loc a) outcome (Decoded c loc a value end))
    : Equal c (decoder_result_end_or c loc a fallback outcome) end =
  J
    (λoutcome2 _.
      Equal
        c
        (decoder_result_end_or c loc a fallback outcome)
        (decoder_result_end_or c loc a fallback outcome2))
    Refl
    succeeded

theorem decoder_remaining_zero_at_equal_end
      (c : Type)
      (el : Type)
      (loc : Type)
      (ops : CursorOps c el loc)
      (start : c)
      (end : c)
      (same_end : Equal c start end)
      (empty_at_start : Equal Nat (cursor_remaining c el loc ops start) Zero)
    : Equal Nat (cursor_remaining c el loc ops end) Zero =
  J (λend2 _. Equal Nat (cursor_remaining c el loc ops end2) Zero) empty_at_start same_end

theorem decoder_nat_elim
      (motive : Nat → Prop) (n : Nat) (zero : motive Zero) (suc : (n2 : Nat) → motive (Suc n2))
    : motive n =
  match n {
    Zero ↦ zero;
    Suc n2 ↦ suc n2
  }

fn decoder_many_zero_result
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (cur : c)
      (remaining : Nat)
    : DecoderResult c loc (List a) =
  match remaining {
    Zero ↦ Decoded c loc (List a) (Nil a) cur;
    Suc rest ↦
      DecoderFailed c loc (List a) (DecoderFuelExhausted loc (cursor_locate c el loc ops cur))
  }

fn decoder_many_decoded_result
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (fuel : Nat)
      (cur : c)
      (value : a)
      (next : c)
      (comparison : Bool)
      (recursive_outcome : DecoderResult c loc (List a))
    : DecoderResult c loc (List a) =
  match comparison {
    False ↦
      DecoderFailed c loc (List a) (DecoderZeroProgress loc (cursor_locate c el loc ops next));
    True ↦
      match recursive_outcome {
        DecoderFailed err ↦ DecoderFailed c loc (List a) err;
        Decoded rest end ↦ Decoded c loc (List a) (Cons a value rest) end
      }
  }

fn decoder_many_fuel_outcome
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (fuel : Nat)
      (cur : c)
      (outcome : DecoderResult c loc a)
    : DecoderResult c loc (List a) =
  match fuel {
    Zero ↦
      match cursor_remaining c el loc ops cur {
        Zero ↦ Decoded c loc (List a) (Nil a) cur;
        Suc rest ↦
          DecoderFailed
            c
            loc
            (List a)
            (DecoderFuelExhausted loc (cursor_locate c el loc ops cur))
      };
    Suc fuel2 ↦
      match outcome {
        DecoderFailed err ↦
          match err {
            DecoderRejected at ↦ Decoded c loc (List a) (Nil a) cur;
            DecoderZeroProgress at ↦ DecoderFailed c loc (List a) (DecoderZeroProgress loc at);
            DecoderFuelExhausted at ↦ DecoderFailed c loc (List a) (DecoderFuelExhausted loc at)
          };
        Decoded value next ↦
          decoder_many_decoded_result
            c
            el
            loc
            a
            ops
            fuel2
            cur
            value
            next
            (cursor_nat_lt
              (cursor_remaining c el loc ops next)
              (cursor_remaining c el loc ops cur))
            (decoder_many_fuel c el loc a ops step fuel2 next)
      }
  }

theorem decoder_equal_after_left_replacement
      (ty : Type)
      (left : ty)
      (replacement : ty)
      (right : ty)
      (left_is_replacement : Equal ty left replacement)
      (left_is_right : Equal ty left right)
    : Equal ty replacement right =
  J (λreplacement2 _. Equal ty replacement2 right) left_is_right left_is_replacement

theorem decoder_many_fuel_outcome_matches
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (fuel : Nat)
      (cur : c)
      (outcome : DecoderResult c loc a)
      (outcome_is_step : Equal (DecoderResult c loc a) (step cur) outcome)
    : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel cur)
        (decoder_many_fuel_outcome c el loc a ops step fuel cur outcome) =
  J
    (λoutcome2 _.
      Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel cur)
        (decoder_many_fuel_outcome c el loc a ops step fuel cur outcome2))
    Refl
    outcome_is_step

theorem decoder_many_outcome_success
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (fuel : Nat)
      (cur : c)
      (outcome : DecoderResult c loc a)
      (outcome_is_step : Equal (DecoderResult c loc a) (step cur) outcome)
      (values : List a)
      (end : c)
      (succeeded : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel cur)
        (Decoded c loc (List a) values end))
    : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel_outcome c el loc a ops step fuel cur outcome)
        (Decoded c loc (List a) values end) =
  decoder_equal_after_left_replacement
    (DecoderResult c loc (List a))
    (decoder_many_fuel c el loc a ops step fuel cur)
    (decoder_many_fuel_outcome c el loc a ops step fuel cur outcome)
    (Decoded c loc (List a) values end)
    (decoder_many_fuel_outcome_matches c el loc a ops step fuel cur outcome outcome_is_step)
    succeeded

theorem decoder_many_zero_result_matches
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (cur : c)
      (remaining : Nat)
      (remaining_is_actual : Equal Nat (cursor_remaining c el loc ops cur) remaining)
    : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step Zero cur)
        (decoder_many_zero_result c el loc a ops cur remaining) =
  J
    (λremaining2 _.
      Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step Zero cur)
        (decoder_many_zero_result c el loc a ops cur remaining2))
    Refl
    remaining_is_actual

theorem decoder_many_zero_success
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (cur : c)
      (remaining : Nat)
      (remaining_is_actual : Equal Nat (cursor_remaining c el loc ops cur) remaining)
      (values : List a)
      (end : c)
      (succeeded : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step Zero cur)
        (Decoded c loc (List a) values end))
    : Equal
        (DecoderResult c loc (List a))
        (decoder_many_zero_result c el loc a ops cur remaining)
        (Decoded c loc (List a) values end) =
  decoder_equal_after_left_replacement
    (DecoderResult c loc (List a))
    (decoder_many_fuel c el loc a ops step Zero cur)
    (decoder_many_zero_result c el loc a ops cur remaining)
    (Decoded c loc (List a) values end)
    (decoder_many_zero_result_matches c el loc a ops step cur remaining remaining_is_actual)
    succeeded

theorem decoder_equal_chain
      (ty : Type)
      (first : ty)
      (middle : ty)
      (last : ty)
      (first_is_middle : Equal ty first middle)
      (middle_is_last : Equal ty middle last)
    : Equal ty first last =
  J (λlast2 _. Equal ty first last2) first_is_middle middle_is_last

theorem decoder_many_decoded_result_matches
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (fuel : Nat)
      (cur : c)
      (value : a)
      (next : c)
      (actual_comparison : Bool)
      (comparison : Bool)
      (actual_recursive : DecoderResult c loc (List a))
      (recursive_outcome : DecoderResult c loc (List a))
      (comparison_is_actual : Equal Bool actual_comparison comparison)
      (recursive_is_actual : Equal
        (DecoderResult c loc (List a))
        actual_recursive
        recursive_outcome)
    : Equal
        (DecoderResult c loc (List a))
        (decoder_many_decoded_result
          c
          el
          loc
          a
          ops
          fuel
          cur
          value
          next
          actual_comparison
          actual_recursive)
        (decoder_many_decoded_result
          c
          el
          loc
          a
          ops
          fuel
          cur
          value
          next
          comparison
          recursive_outcome) =
  let
    comparison_changed : Equal
      (DecoderResult c loc (List a))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        actual_comparison
        actual_recursive)
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        actual_recursive) =
      J
        (λcomparison2 _.
          Equal
            (DecoderResult c loc (List a))
            (decoder_many_decoded_result
              c
              el
              loc
              a
              ops
              fuel
              cur
              value
              next
              actual_comparison
              actual_recursive)
            (decoder_many_decoded_result
              c
              el
              loc
              a
              ops
              fuel
              cur
              value
              next
              comparison2
              actual_recursive))
        Refl
        comparison_is_actual;
    recursive_changed : Equal
      (DecoderResult c loc (List a))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        actual_recursive)
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        recursive_outcome) =
      J
        (λrecursive2 _.
          Equal
            (DecoderResult c loc (List a))
            (decoder_many_decoded_result
              c
              el
              loc
              a
              ops
              fuel
              cur
              value
              next
              comparison
              actual_recursive)
            (decoder_many_decoded_result
              c
              el
              loc
              a
              ops
              fuel
              cur
              value
              next
              comparison
              recursive2))
        Refl
        recursive_is_actual
  in
    decoder_equal_chain
      (DecoderResult c loc (List a))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        actual_comparison
        actual_recursive)
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        actual_recursive)
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        recursive_outcome)
      comparison_changed
      recursive_changed

theorem decoder_many_decoded_success
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (fuel : Nat)
      (cur : c)
      (value : a)
      (next : c)
      (comparison : Bool)
      (recursive_outcome : DecoderResult c loc (List a))
      (comparison_is_actual : Equal
        Bool
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        comparison)
      (recursive_is_actual : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel next)
        recursive_outcome)
      (step_succeeded : Equal (DecoderResult c loc a) (step cur) (Decoded c loc a value next))
      (values : List a)
      (end : c)
      (succeeded : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step (Suc fuel) cur)
        (Decoded c loc (List a) values end))
    : Equal
        (DecoderResult c loc (List a))
        (decoder_many_decoded_result
          c
          el
          loc
          a
          ops
          fuel
          cur
          value
          next
          comparison
          recursive_outcome)
        (Decoded c loc (List a) values end) =
  let
    matched_success : Equal
      (DecoderResult c loc (List a))
      (decoder_many_fuel_outcome
        c
        el
        loc
        a
        ops
        step
        (Suc fuel)
        cur
        (Decoded c loc a value next))
      (Decoded c loc (List a) values end) =
      decoder_many_outcome_success
        c
        el
        loc
        a
        ops
        step
        (Suc fuel)
        cur
        (Decoded c loc a value next)
        step_succeeded
        values
        end
        succeeded;
    decoded_result_matches : Equal
      (DecoderResult c loc (List a))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        (decoder_many_fuel c el loc a ops step fuel next))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        recursive_outcome) =
      decoder_many_decoded_result_matches
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        comparison
        (decoder_many_fuel c el loc a ops step fuel next)
        recursive_outcome
        comparison_is_actual
        recursive_is_actual
  in
    decoder_equal_after_left_replacement
      (DecoderResult c loc (List a))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        (decoder_many_fuel c el loc a ops step fuel next))
      (decoder_many_decoded_result
        c
        el
        loc
        a
        ops
        fuel
        cur
        value
        next
        comparison
        recursive_outcome)
      (Decoded c loc (List a) values end)
      decoded_result_matches
      matched_success

theorem decoder_many_decoded_false_consumes_all
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (fuel : Nat)
      (cur : c)
      (value : a)
      (next : c)
      (recursive_outcome : DecoderResult c loc (List a))
      (comparison_is_actual : Equal
        Bool
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        False)
      (recursive_is_actual : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel next)
        recursive_outcome)
      (step_succeeded : Equal (DecoderResult c loc a) (step cur) (Decoded c loc a value next))
      (progress : DecoderProgress c el loc a ops step)
      (recurse : (next_cur : c)
        → (next_values : List a)
        → (next_end : c)
        → Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel next_cur)
        (Decoded c loc (List a) next_values next_end)
        → Equal
        Nat
        (cursor_remaining c el loc ops next_end)
        Zero)
      (values : List a)
      (end : c)
      (succeeded : Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step (Suc fuel) cur)
        (Decoded c loc (List a) values end))
    : Equal Nat (cursor_remaining c el loc ops end) Zero =
  absurd
    (decoder_equal_after_left_replacement
      Bool
      (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
      False
      True
      comparison_is_actual
      (progress cur value next step_succeeded))

theorem decoder_many_decoded_true_consumes_all
    : (c : Type)
      → (el : Type)
      → (loc : Type)
      → (a : Type)
      → (ops : CursorOps c el loc)
      → (step : Decoder c loc a)
      → (fuel : Nat)
      → (cur : c)
      → (value : a)
      → (next : c)
      → (recursive_outcome : DecoderResult c loc (List a))
      → Equal Bool
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        True
      → Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel next)
        recursive_outcome
      → Equal (DecoderResult c loc a) (step cur) (Decoded c loc a value next)
      → DecoderProgress c el loc a ops step
      → ((next_cur : c)
          → (next_values : List a)
          → (next_end : c)
          → Equal
          (DecoderResult c loc (List a))
          (decoder_many_fuel c el loc a ops step fuel next_cur)
          (Decoded c loc (List a) next_values next_end)
          → Equal
          Nat
          (cursor_remaining c el loc ops next_end)
          Zero)
      → (values : List a)
      → (end : c)
      → Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step (Suc fuel) cur)
        (Decoded c loc (List a) values end)
      → Equal Nat (cursor_remaining c el loc ops end) Zero =
  λc.
    λel.
      λloc.
        λa.
          λops.
            λstep.
              λfuel.
                λcur.
                  λvalue.
                    λnext.
                      λrecursive_outcome.
                        λcomparison_is_actual.
                          decoder_result_elim
                            c
                            loc
                            (List a)
                            (λrecursive2.
                              Equal
                                (DecoderResult c loc (List a))
                                (decoder_many_fuel c el loc a ops step fuel next)
                                recursive2
                              → Equal
                                (DecoderResult c loc a)
                                (step cur)
                                (Decoded c loc a value next)
                              → DecoderProgress c el loc a ops step
                              → ((next_cur : c)
                                → (next_values : List a)
                                → (next_end : c)
                                → Equal
                                (DecoderResult c loc (List a))
                                (decoder_many_fuel c el loc a ops step fuel next_cur)
                                (Decoded c loc (List a) next_values next_end)
                                → Equal
                                Nat
                                (cursor_remaining c el loc ops next_end)
                                Zero)
                              → (values : List a)
                                → (end : c)
                                → Equal
                                (DecoderResult c loc (List a))
                                (decoder_many_fuel c el loc a ops step (Suc fuel) cur)
                                (Decoded c loc (List a) values end)
                                → Equal Nat
                                (cursor_remaining c el loc ops end)
                                Zero)
                            recursive_outcome
                            (λrest.
                              λfinal.
                                λrecursive_is_actual.
                                  λstep_succeeded.
                                    λprogress.
                                      λrecurse.
                                        λvalues.
                                          λend.
                                            λsucceeded.
                                              let
                                                matched_success : Equal
                                                  (DecoderResult c loc (List a))
                                                  (decoder_many_decoded_result
                                                    c
                                                    el
                                                    loc
                                                    a
                                                    ops
                                                    fuel
                                                    cur
                                                    value
                                                    next
                                                    True
                                                    (Decoded c loc (List a) rest final))
                                                  (Decoded c loc (List a) values end) =
                                                  decoder_many_decoded_success
                                                    c
                                                    el
                                                    loc
                                                    a
                                                    ops
                                                    step
                                                    fuel
                                                    cur
                                                    value
                                                    next
                                                    True
                                                    (Decoded c loc (List a) rest final)
                                                    comparison_is_actual
                                                    recursive_is_actual
                                                    step_succeeded
                                                    values
                                                    end
                                                    succeeded;
                                                empty_at_final : Equal Nat
                                                  (cursor_remaining c el loc ops final)
                                                  Zero =
                                                  recurse next rest final recursive_is_actual
                                              in
                                                decoder_remaining_zero_at_equal_end
                                                  c
                                                  el
                                                  loc
                                                  ops
                                                  final
                                                  end
                                                  (decoder_success_end
                                                    c
                                                    loc
                                                    (List a)
                                                    cur
                                                    (decoder_many_decoded_result
                                                      c
                                                      el
                                                      loc
                                                      a
                                                      ops
                                                      fuel
                                                      cur
                                                      value
                                                      next
                                                      True
                                                      (Decoded c loc (List a) rest final))
                                                    values
                                                    end
                                                    matched_success)
                                                  empty_at_final)
                            (λerr.
                              λrecursive_is_actual.
                                λstep_succeeded.
                                  λprogress.
                                    λrecurse.
                                      λvalues.
                                        λend.
                                          λsucceeded.
                                            absurd
                                              (decoder_many_decoded_success
                                                c
                                                el
                                                loc
                                                a
                                                ops
                                                step
                                                fuel
                                                cur
                                                value
                                                next
                                                True
                                                (DecoderFailed c loc (List a) err)
                                                comparison_is_actual
                                                recursive_is_actual
                                                step_succeeded
                                                values
                                                end
                                                succeeded))

theorem decoder_many_decoded_consumes_all
      (comparison : Bool)
    : (c : Type)
      → (el : Type)
      → (loc : Type)
      → (a : Type)
      → (ops : CursorOps c el loc)
      → (step : Decoder c loc a)
      → (fuel : Nat)
      → (cur : c)
      → (value : a)
      → (next : c)
      → (recursive_outcome : DecoderResult c loc (List a))
      → Equal Bool
        (cursor_nat_lt (cursor_remaining c el loc ops next) (cursor_remaining c el loc ops cur))
        comparison
      → Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel next)
        recursive_outcome
      → Equal (DecoderResult c loc a) (step cur) (Decoded c loc a value next)
      → DecoderProgress c el loc a ops step
      → ((next_cur : c)
          → (next_values : List a)
          → (next_end : c)
          → Equal
          (DecoderResult c loc (List a))
          (decoder_many_fuel c el loc a ops step fuel next_cur)
          (Decoded c loc (List a) next_values next_end)
          → Equal
          Nat
          (cursor_remaining c el loc ops next_end)
          Zero)
      → (values : List a)
      → (end : c)
      → Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step (Suc fuel) cur)
        (Decoded c loc (List a) values end)
      → Equal Nat (cursor_remaining c el loc ops end) Zero =
  match comparison {
    False ↦
      λc.
        λel.
          λloc.
            λa.
              λops.
                λstep.
                  λfuel.
                    λcur.
                      λvalue.
                        λnext.
                          λrecursive_outcome.
                            decoder_many_decoded_false_consumes_all
                              c
                              el
                              loc
                              a
                              ops
                              step
                              fuel
                              cur
                              value
                              next
                              recursive_outcome;
    True ↦
      λc.
        λel.
          λloc.
            λa.
              λops.
                λstep.
                  λfuel.
                    λcur.
                      λvalue.
                        λnext.
                          λrecursive_outcome.
                            decoder_many_decoded_true_consumes_all
                              c
                              el
                              loc
                              a
                              ops
                              step
                              fuel
                              cur
                              value
                              next
                              recursive_outcome
  }

theorem decoder_many_fuel_consumes_all
      (fuel : Nat)
    : (c : Type)
      → (el : Type)
      → (loc : Type)
      → (a : Type)
      → (ops : CursorOps c el loc)
      → (step : Decoder c loc a)
      → DecoderProgress c el loc a ops step
      → DecoderRejectsOnlyAtEnd c el loc a ops step
      → (cur : c)
      → (outcome : DecoderResult c loc a)
      → Equal (DecoderResult c loc a) (step cur) outcome
      → (values : List a)
      → (end : c)
      → Equal
        (DecoderResult c loc (List a))
        (decoder_many_fuel c el loc a ops step fuel cur)
        (Decoded c loc (List a) values end)
      → Equal Nat (cursor_remaining c el loc ops end) Zero =
  match fuel {
    Zero ↦
      λc.
        λel.
          λloc.
            λa.
              λops.
                λstep.
                  λprogress.
                    λrejects.
                      λcur.
                        λoutcome.
                          λoutcome_is_step.
                            decoder_nat_elim
                              (λremaining.
                                Equal Nat (cursor_remaining c el loc ops cur) remaining
                                → (values : List a)
                                  → (end : c)
                                  → Equal
                                  (DecoderResult c loc (List a))
                                  (decoder_many_fuel c el loc a ops step Zero cur)
                                  (Decoded c loc (List a) values end)
                                  → Equal Nat
                                  (cursor_remaining c el loc ops end)
                                  Zero)
                              (cursor_remaining c el loc ops cur)
                              (λremaining_is_actual.
                                λvalues.
                                  λend.
                                    λsucceeded.
                                      let matched_success : Equal
                                        (DecoderResult c loc (List a))
                                        (decoder_many_zero_result c el loc a ops cur Zero)
                                        (Decoded c loc (List a) values end) =
                                        decoder_many_zero_success
                                          c
                                          el
                                          loc
                                          a
                                          ops
                                          step
                                          cur
                                          Zero
                                          remaining_is_actual
                                          values
                                          end
                                          succeeded
                                      in
                                        decoder_remaining_zero_at_equal_end
                                          c
                                          el
                                          loc
                                          ops
                                          cur
                                          end
                                          (decoder_success_end
                                            c
                                            loc
                                            (List a)
                                            cur
                                            (decoder_many_zero_result c el loc a ops cur Zero)
                                            values
                                            end
                                            matched_success)
                                          remaining_is_actual)
                              (λrest.
                                λremaining_is_actual.
                                  λvalues.
                                    λend.
                                      λsucceeded.
                                        absurd
                                          (decoder_many_zero_success
                                            c
                                            el
                                            loc
                                            a
                                            ops
                                            step
                                            cur
                                            (Suc rest)
                                            remaining_is_actual
                                            values
                                            end
                                            succeeded))
                              Refl;
    Suc fuel2 ↦
      λc.
        λel.
          λloc.
            λa.
              λops.
                λstep.
                  λprogress.
                    λrejects.
                      λcur.
                        λoutcome.
                          decoder_result_elim
                            c
                            loc
                            a
                            (λoutcome2.
                              Equal (DecoderResult c loc a) (step cur) outcome2
                              → (values : List a)
                                → (end : c)
                                → Equal
                                (DecoderResult c loc (List a))
                                (decoder_many_fuel c el loc a ops step (Suc fuel2) cur)
                                (Decoded c loc (List a) values end)
                                → Equal Nat
                                (cursor_remaining c el loc ops end)
                                Zero)
                            outcome
                            (λvalue.
                              λnext.
                                λoutcome_is_step.
                                  let recurse : (next_cur : c)
                                    → (next_values : List a)
                                    → (next_end : c)
                                    → Equal
                                    (DecoderResult c loc (List a))
                                    (decoder_many_fuel c el loc a ops step fuel2 next_cur)
                                    (Decoded c loc (List a) next_values next_end)
                                    → Equal Nat
                                    (cursor_remaining c el loc ops next_end)
                                    Zero =
                                    λnext_cur.
                                      λnext_values.
                                        λnext_end.
                                          λrecursive_succeeded.
                                            decoder_many_fuel_consumes_all
                                              fuel2
                                              c
                                              el
                                              loc
                                              a
                                              ops
                                              step
                                              progress
                                              rejects
                                              next_cur
                                              (step next_cur)
                                              Refl
                                              next_values
                                              next_end
                                              recursive_succeeded
                                  in
                                    decoder_many_decoded_consumes_all
                                      (cursor_nat_lt
                                        (cursor_remaining c el loc ops next)
                                        (cursor_remaining c el loc ops cur))
                                      c
                                      el
                                      loc
                                      a
                                      ops
                                      step
                                      fuel2
                                      cur
                                      value
                                      next
                                      (decoder_many_fuel c el loc a ops step fuel2 next)
                                      Refl
                                      Refl
                                      outcome_is_step
                                      progress
                                      recurse)
                            (λerr.
                              decoder_error_elim
                                loc
                                (λerr2.
                                  Equal
                                    (DecoderResult c loc a)
                                    (step cur)
                                    (DecoderFailed c loc a err2)
                                  → (values : List a)
                                    → (end : c)
                                    → Equal
                                    (DecoderResult c loc (List a))
                                    (decoder_many_fuel c el loc a ops step (Suc fuel2) cur)
                                    (Decoded c loc (List a) values end)
                                    → Equal Nat
                                    (cursor_remaining c el loc ops end)
                                    Zero)
                                err
                                (λat.
                                  λoutcome_is_step.
                                    λvalues.
                                      λend.
                                        λsucceeded.
                                          let matched_success : Equal
                                            (DecoderResult c loc (List a))
                                            (decoder_many_fuel_outcome
                                              c
                                              el
                                              loc
                                              a
                                              ops
                                              step
                                              (Suc fuel2)
                                              cur
                                              (DecoderFailed c loc a (DecoderRejected loc at)))
                                            (Decoded c loc (List a) values end) =
                                            decoder_many_outcome_success
                                              c
                                              el
                                              loc
                                              a
                                              ops
                                              step
                                              (Suc fuel2)
                                              cur
                                              (DecoderFailed c loc a (DecoderRejected loc at))
                                              outcome_is_step
                                              values
                                              end
                                              succeeded
                                          in
                                            decoder_remaining_zero_at_equal_end
                                              c
                                              el
                                              loc
                                              ops
                                              cur
                                              end
                                              (decoder_success_end
                                                c
                                                loc
                                                (List a)
                                                cur
                                                (decoder_many_fuel_outcome
                                                  c
                                                  el
                                                  loc
                                                  a
                                                  ops
                                                  step
                                                  (Suc fuel2)
                                                  cur
                                                  (DecoderFailed
                                                    c
                                                    loc
                                                    a
                                                    (DecoderRejected loc at)))
                                                values
                                                end
                                                matched_success)
                                              (rejects cur at outcome_is_step))
                                (λat.
                                  λoutcome_is_step.
                                    λvalues.
                                      λend.
                                        λsucceeded.
                                          absurd
                                            (decoder_many_outcome_success
                                              c
                                              el
                                              loc
                                              a
                                              ops
                                              step
                                              (Suc fuel2)
                                              cur
                                              (DecoderFailed
                                                c
                                                loc
                                                a
                                                (DecoderZeroProgress loc at))
                                              outcome_is_step
                                              values
                                              end
                                              succeeded))
                                (λat.
                                  λoutcome_is_step.
                                    λvalues.
                                      λend.
                                        λsucceeded.
                                          absurd
                                            (decoder_many_outcome_success
                                              c
                                              el
                                              loc
                                              a
                                              ops
                                              step
                                              (Suc fuel2)
                                              cur
                                              (DecoderFailed
                                                c
                                                loc
                                                a
                                                (DecoderFuelExhausted loc at))
                                              outcome_is_step
                                              values
                                              end
                                              succeeded)))
  }

theorem decoder_many_consumes_all
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
    : DecoderManyConsumesAllLaw c el loc a ops step =
  λlaws.
    let
      progress : DecoderProgress c el loc a ops step =
        and_fst
          (DecoderProgress c el loc a ops step)
          (DecoderRejectsOnlyAtEnd c el loc a ops step)
          laws;
      rejects : DecoderRejectsOnlyAtEnd c el loc a ops step =
        and_snd
          (DecoderProgress c el loc a ops step)
          (DecoderRejectsOnlyAtEnd c el loc a ops step)
          laws
    in
      λcur.
        λvalues.
          λend.
            λsucceeded.
              decoder_many_fuel_consumes_all
                (cursor_remaining c el loc ops cur)
                c
                el
                loc
                a
                ops
                step
                progress
                rejects
                cur
                (step cur)
                Refl
                values
                end
                succeeded
```

### Invariant preservation

A client chooses predicates for well-formed cursors and locations. Successful
decodes retain the cursor predicate; every failed decode satisfies the location
predicate at the error's public location. The base and composite laws below
are checked beside the private case splits and fuel recursors. Their premises
express cursor-location soundness, not assumptions about a particular client.
`DecoderNonBacktrackable` distinguishes ordinary rejection from the two
non-backtrackable errors without publishing their constructors. The two
`alt` equations establish the actual branch choice: rejection tries the
second decoder; non-backtrackable failure keeps the first error. Preservation
alone would also hold for an incorrect fallback on zero progress, so the
branch equations are separate checked claims.

```ken
fn DecoderResultPreserved
      (c : Type)
      (loc : Type)
      (a : Type)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (outcome : DecoderResult c loc a)
    : Prop =
  match outcome {
    Decoded value next ↦ good_cursor next;
    DecoderFailed err ↦ good_location (decoder_error_location loc err)
  }

pub fn DecoderNonBacktrackable (loc : Type) (err : DecoderError loc) : Prop =
  match err {
    DecoderRejected at ↦ Bottom;
    DecoderZeroProgress at ↦ Top;
    DecoderFuelExhausted at ↦ Top
  }

pub fn DecoderPreserves
      (c : Type)
      (loc : Type)
      (a : Type)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (decoder : Decoder c loc a)
    : Prop =
  (cur : c)
    → good_cursor cur → DecoderResultPreserved c loc a good_cursor good_location
    (decoder cur)

pub theorem decoder_pure_preserves
      (c : Type)
      (loc : Type)
      (a : Type)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (value : a)
    : DecoderPreserves c loc a good_cursor good_location (decoder_pure c loc a value) =
  λcur. λgood. good

pub theorem decoder_fail_preserves
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
    : DecoderPreserves c loc a good_cursor good_location (decoder_fail c el loc a ops) =
  λcur. λgood. locate_sound cur good

theorem decoder_zero_progress_nonbacktrackable
      (loc : Type) (at : loc)
    : DecoderNonBacktrackable loc (DecoderZeroProgress loc at) =
  Proved

theorem decoder_fuel_exhausted_nonbacktrackable
      (loc : Type) (at : loc)
    : DecoderNonBacktrackable loc (DecoderFuelExhausted loc at) =
  Proved

theorem decoder_alt_nonbacktrackable_error
      (c : Type)
      (loc : Type)
      (a : Type)
      (second : Decoder c loc a)
      (cur : c)
      (err : DecoderError loc)
      (nonbacktrackable : DecoderNonBacktrackable loc err)
    : Equal
        (DecoderResult c loc a)
        (decoder_alt_result c loc a second cur (DecoderFailed c loc a err))
        (DecoderFailed c loc a err) =
  decoder_error_elim
    loc
    (λerror.
      DecoderNonBacktrackable loc error
      → Equal
        (DecoderResult c loc a)
        (decoder_alt_result c loc a second cur (DecoderFailed c loc a error))
        (DecoderFailed c loc a error))
    err
    (λat. λimpossible. absurd impossible)
    (λat. λvalid. Refl)
    (λat. λvalid. Refl)
    nonbacktrackable

pub theorem decoder_alt_propagates_nonbacktrackable
      (c : Type)
      (loc : Type)
      (a : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc a)
      (cur : c)
      (err : DecoderError loc)
      (nonbacktrackable : DecoderNonBacktrackable loc err)
      (first_failed : Equal (DecoderResult c loc a) (first cur) (DecoderFailed c loc a err))
    : Equal
        (DecoderResult c loc a)
        (decoder_alt c loc a first second cur)
        (DecoderFailed c loc a err) =
  decoder_equal_chain
    (DecoderResult c loc a)
    (decoder_alt_result c loc a second cur (first cur))
    (decoder_alt_result c loc a second cur (DecoderFailed c loc a err))
    (DecoderFailed c loc a err)
    (J
      (λoutcome _.
        Equal
          (DecoderResult c loc a)
          (decoder_alt_result c loc a second cur (first cur))
          (decoder_alt_result c loc a second cur outcome))
      Refl
      first_failed)
    (decoder_alt_nonbacktrackable_error c loc a second cur err nonbacktrackable)

pub theorem decoder_alt_rejection_uses_second
      (c : Type)
      (loc : Type)
      (a : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc a)
      (cur : c)
      (at : loc)
      (first_rejected : Equal
        (DecoderResult c loc a)
        (first cur)
        (DecoderFailed c loc a (DecoderRejected loc at)))
    : Equal (DecoderResult c loc a) (decoder_alt c loc a first second cur) (second cur) =
  J
    (λoutcome _.
      Equal
        (DecoderResult c loc a)
        (decoder_alt_result c loc a second cur (first cur))
        (decoder_alt_result c loc a second cur outcome))
    Refl
    first_rejected

pub theorem decoder_bind_preserves
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (first : Decoder c loc a)
      (next_decoder : a → Decoder c loc b)
      (first_preserves : DecoderPreserves c loc a good_cursor good_location first)
      (next_preserves : (value : a)
        → DecoderPreserves
        c
        loc
        b
        good_cursor
        good_location
        (next_decoder value))
    : DecoderPreserves c loc b good_cursor good_location
        (decoder_bind c loc a b first next_decoder) =
  λcur.
    λgood.
      decoder_result_elim
        c
        loc
        a
        (λfirst_outcome.
          DecoderResultPreserved c loc a good_cursor good_location first_outcome
          → DecoderResultPreserved
            c
            loc
            b
            good_cursor
            good_location
            (decoder_bind_result c loc a b next_decoder first_outcome))
        (first cur)
        (λvalue. λnext. λvalid. next_preserves value next valid)
        (λerr. λvalid. valid)
        (first_preserves cur good)

pub theorem decoder_seq_preserves
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (first : Decoder c loc a)
      (second : Decoder c loc b)
      (first_preserves : DecoderPreserves c loc a good_cursor good_location first)
      (second_preserves : DecoderPreserves c loc b good_cursor good_location second)
    : DecoderPreserves c loc b good_cursor good_location (decoder_seq c loc a b first second) =
  decoder_bind_preserves
    c
    loc
    a
    b
    good_cursor
    good_location
    first
    (λignored. second)
    first_preserves
    (λignored. second_preserves)

pub theorem decoder_alt_preserves
      (c : Type)
      (loc : Type)
      (a : Type)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (first : Decoder c loc a)
      (second : Decoder c loc a)
      (first_preserves : DecoderPreserves c loc a good_cursor good_location first)
      (second_preserves : DecoderPreserves c loc a good_cursor good_location second)
    : DecoderPreserves c loc a good_cursor good_location (decoder_alt c loc a first second) =
  λcur.
    λgood.
      decoder_result_elim
        c
        loc
        a
        (λfirst_outcome.
          DecoderResultPreserved c loc a good_cursor good_location first_outcome
          → DecoderResultPreserved
            c
            loc
            a
            good_cursor
            good_location
            (decoder_alt_result c loc a second cur first_outcome))
        (first cur)
        (λvalue. λnext. λvalid. valid)
        (λerr.
          decoder_error_elim
            loc
            (λerror.
              DecoderResultPreserved
                c
                loc
                a
                good_cursor
                good_location
                (DecoderFailed c loc a error)
              → DecoderResultPreserved
                c
                loc
                a
                good_cursor
                good_location
                (decoder_alt_result c loc a second cur (DecoderFailed c loc a error)))
            err
            (λat. λvalid. second_preserves cur good)
            (λat. λvalid. valid)
            (λat. λvalid. valid))
        (first_preserves cur good)

theorem decoder_option_prop_elim
      (a : Type)
      (motive : Option a → Prop)
      (option : Option a)
      (none : motive (None a))
      (some : (value : a) → motive (Some a value))
    : motive option =
  match option {
    None ↦ none;
    Some value ↦ some value
  }

theorem decoder_bool_prop_elim
      (motive : Bool → Prop) (value : Bool) (on_true : motive True) (on_false : motive False)
    : motive value =
  match value {
    True ↦ on_true;
    False ↦ on_false
  }

fn decoder_satisfy_accepted
      (c : Type)
      (el : Type)
      (loc : Type)
      (ops : CursorOps c el loc)
      (cur : c)
      (value : el)
      (accepted : Bool)
    : DecoderResult c loc el =
  match accepted {
    True ↦ Decoded c loc el value (cursor_advance c el loc ops cur);
    False ↦ DecoderFailed c loc el (DecoderRejected loc (cursor_locate c el loc ops cur))
  }

fn decoder_satisfy_outcome
      (c : Type)
      (el : Type)
      (loc : Type)
      (ops : CursorOps c el loc)
      (accept : el → Bool)
      (cur : c)
      (observed : Option el)
    : DecoderResult c loc el =
  match observed {
    None ↦ DecoderFailed c loc el (DecoderRejected loc (cursor_locate c el loc ops cur));
    Some value ↦ decoder_satisfy_accepted c el loc ops cur value (accept value)
  }

pub theorem decoder_satisfy_preserves
      (c : Type)
      (el : Type)
      (loc : Type)
      (ops : CursorOps c el loc)
      (accept : el → Bool)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
      (advance_sound : (cur : c)
        → (value : el)
        → Equal
        (Option el)
        (cursor_peek c el loc ops cur)
        (Some el value)
        → good_cursor
        cur
        → good_cursor
        (cursor_advance c el loc ops cur))
    : DecoderPreserves c loc el good_cursor good_location
        (decoder_satisfy c el loc ops accept) =
  λcur.
    λgood.
      decoder_option_prop_elim
        el
        (λobserved.
          Equal (Option el) (cursor_peek c el loc ops cur) observed
          → DecoderResultPreserved
            c
            loc
            el
            good_cursor
            good_location
            (decoder_satisfy_outcome c el loc ops accept cur observed))
        (cursor_peek c el loc ops cur)
        (λnot_found. locate_sound cur good)
        (λvalue.
          λpeeked.
            decoder_bool_prop_elim
              (λaccepted.
                DecoderResultPreserved
                  c
                  loc
                  el
                  good_cursor
                  good_location
                  (decoder_satisfy_accepted c el loc ops cur value accepted))
              (accept value)
              (advance_sound cur value peeked good)
              (locate_sound cur good))
        Refl

theorem decoder_many_fuel_zero_preserves
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
      (cur : c)
      (good : good_cursor cur)
    : DecoderResultPreserved c loc
        (List a)
        good_cursor good_location
        (decoder_many_fuel c el loc a ops step Zero cur) =
  decoder_nat_elim
    (λremaining.
      Equal Nat (cursor_remaining c el loc ops cur) remaining
      → DecoderResultPreserved
        c
        loc
        (List a)
        good_cursor
        good_location
        (decoder_many_zero_result c el loc a ops cur remaining))
    (cursor_remaining c el loc ops cur)
    (λempty. good)
    (λrest. λpositive. locate_sound cur good)
    Refl

theorem decoder_many_decoded_success_preserves
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (fuel : Nat)
      (cur : c)
      (value : a)
      (next : c)
      (recursive_outcome : DecoderResult c loc (List a))
      (recursive_preserved : DecoderResultPreserved
        c
        loc
        (List a)
        good_cursor
        good_location
        recursive_outcome)
    : DecoderResultPreserved c loc
        (List a)
        good_cursor good_location
        (decoder_many_decoded_result
          c
          el
          loc
          a
          ops
          fuel
          cur
          value
          next
          True
          recursive_outcome) =
  decoder_result_elim
    c
    loc
    (List a)
    (λoutcome.
      DecoderResultPreserved c loc (List a) good_cursor good_location outcome
      → DecoderResultPreserved
        c
        loc
        (List a)
        good_cursor
        good_location
        (decoder_many_decoded_result c el loc a ops fuel cur value next True outcome))
    recursive_outcome
    (λrest. λend. λvalid. valid)
    (λerr. λvalid. valid)
    recursive_preserved

theorem decoder_many_fuel_preserves
      (fuel : Nat)
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
      (step_preserves : DecoderPreserves c loc a good_cursor good_location step)
    : (cur : c)
      → good_cursor cur
      → DecoderResultPreserved c loc
        (List a)
        good_cursor good_location
        (decoder_many_fuel c el loc a ops step fuel cur) =
  match fuel {
    Zero ↦
      λcur.
        λgood.
          decoder_many_fuel_zero_preserves
            c
            el
            loc
            a
            ops
            step
            good_cursor
            good_location
            locate_sound
            cur
            good;
    Suc fuel2 ↦
      λcur.
        λgood.
          decoder_result_elim
            c
            loc
            a
            (λstep_outcome.
              DecoderResultPreserved c loc a good_cursor good_location step_outcome
              → DecoderResultPreserved
                c
                loc
                (List a)
                good_cursor
                good_location
                (decoder_many_fuel_outcome c el loc a ops step (Suc fuel2) cur step_outcome))
            (step cur)
            (λvalue.
              λnext.
                λnext_good.
                  decoder_bool_prop_elim
                    (λcomparison.
                      DecoderResultPreserved
                        c
                        loc
                        (List a)
                        good_cursor
                        good_location
                        (decoder_many_decoded_result
                          c
                          el
                          loc
                          a
                          ops
                          fuel2
                          cur
                          value
                          next
                          comparison
                          (decoder_many_fuel c el loc a ops step fuel2 next)))
                    (cursor_nat_lt
                      (cursor_remaining c el loc ops next)
                      (cursor_remaining c el loc ops cur))
                    (decoder_many_decoded_success_preserves
                      c
                      el
                      loc
                      a
                      ops
                      good_cursor
                      good_location
                      fuel2
                      cur
                      value
                      next
                      (decoder_many_fuel c el loc a ops step fuel2 next)
                      (decoder_many_fuel_preserves
                        fuel2
                        c
                        el
                        loc
                        a
                        ops
                        step
                        good_cursor
                        good_location
                        locate_sound
                        step_preserves
                        next
                        next_good))
                    (locate_sound next next_good))
            (λerr.
              decoder_error_elim
                loc
                (λerror.
                  good_location (decoder_error_location loc error)
                  → DecoderResultPreserved
                    c
                    loc
                    (List a)
                    good_cursor
                    good_location
                    (decoder_many_fuel_outcome
                      c
                      el
                      loc
                      a
                      ops
                      step
                      (Suc fuel2)
                      cur
                      (DecoderFailed c loc a error)))
                err
                (λat. λvalid. good)
                (λat. λvalid. valid)
                (λat. λvalid. valid))
            (step_preserves cur good)
  }

pub theorem decoder_many_preserves
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (step : Decoder c loc a)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
      (step_preserves : DecoderPreserves c loc a good_cursor good_location step)
    : DecoderPreserves c loc
        (List a)
        good_cursor good_location
        (decoder_many c el loc a ops step) =
  λcur.
    λgood.
      decoder_many_fuel_preserves
        (cursor_remaining c el loc ops cur)
        c
        el
        loc
        a
        ops
        step
        good_cursor
        good_location
        locate_sound
        step_preserves
        cur
        good

theorem decoder_recursive_fuel_preserves
      (fuel : Nat)
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (layer : Decoder c loc a → Decoder c loc a)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
      (layer_preserves : (recur : Decoder c loc a)
        → DecoderPreserves
        c
        loc
        a
        good_cursor
        good_location
        recur
        → DecoderPreserves
        c
        loc
        a
        good_cursor
        good_location
        (layer recur))
    : DecoderPreserves c loc a good_cursor good_location
        (decoder_recursive_fuel c el loc a ops layer fuel) =
  match fuel {
    Zero ↦ λcur. λgood. locate_sound cur good;
    Suc fuel2 ↦
      layer_preserves
        (decoder_recursive_fuel c el loc a ops layer fuel2)
        (decoder_recursive_fuel_preserves
          fuel2
          c
          el
          loc
          a
          ops
          layer
          good_cursor
          good_location
          locate_sound
          layer_preserves)
  }

pub theorem decoder_recursive_preserves
      (c : Type)
      (el : Type)
      (loc : Type)
      (a : Type)
      (ops : CursorOps c el loc)
      (layer : Decoder c loc a → Decoder c loc a)
      (good_cursor : c → Prop)
      (good_location : loc → Prop)
      (locate_sound : (cur : c)
        → good_cursor
        cur
        → good_location
        (cursor_locate c el loc ops cur))
      (layer_preserves : (recur : Decoder c loc a)
        → DecoderPreserves
        c
        loc
        a
        good_cursor
        good_location
        recur
        → DecoderPreserves
        c
        loc
        a
        good_cursor
        good_location
        (layer recur))
    : DecoderPreserves c loc a good_cursor good_location
        (decoder_recursive c el loc a ops layer) =
  λcur.
    λgood.
      decoder_recursive_fuel_preserves
        (cursor_remaining c el loc ops cur)
        c
        el
        loc
        a
        ops
        layer
        good_cursor
        good_location
        locate_sound
        layer_preserves
        cur
        good
```

## 3. Using it

Build token decoders with `decoder_satisfy`, use `decoder_pure` and
`decoder_fail` as the base cases, and combine them with `decoder_bind`,
`decoder_seq`, and `decoder_alt`. `decoder_many` repeats a step, while
`decoder_recursive` supplies a structurally fuel-bounded recursive layer.
Callers never supply repetition or recursion fuel; both bounds come from
`CursorOps.remaining`. A caller may apply `DecoderPreserves` with its own
cursor/location predicates. For `satisfy` it supplies an advance-preservation
premise for successful peeks; for `many` it supplies step preservation; and
for `recursive` it supplies preservation of the layer for every preserving
recursive argument. The latter two laws discharge the private fuel cases
internally rather than exposing a caller fuel budget.

## 4. Design notes

`DecoderFuelExhausted` is observable only when a cursor or recursive layer
violates its stated progress contract. Legal repeated input cannot reach it:
every success consumes at least one unit from a fuel seed equal to `remaining`.

## 5. References

None.

## 6. Trust  derivation

Every combinator is transparent, structurally recursive on `Nat` fuel, and
uses only checked cursor operations. The semantic equations, repetition law, and parametric preservation laws are
ordinary transparent terms using `J`, structural eliminators, and the prelude
conjunction projections; they import no proof assumption. In particular,
`decoder_many_preserves` inducts over its private fuel and keeps zero-progress
and fuel-exhaustion locations valid; `decoder_recursive_preserves` inducts over
its private fuel and uses a layer-preservation premise. Neither this package
nor its proof dependencies add an axiom or primitive.

## 7. Package  summary

Public surface: `DecoderError` with ordinary `DecoderRejected`, `DecoderResult`
with `Decoded` and `DecoderFailed`, `Decoder`, location projection, and the
pure, failure, bind, sequence, alternative, predicate-token, repetition, and
recursive combinators consumed by downstream packages. `DecoderPreserves`,
`DecoderNonBacktrackable`, the two `alt` branch equations, and the eight
combinator preservation theorems form the checked public proof interface.
The older semantic equations, `DecoderManyConsumesAllLaw`, and both fuel
recursors stay private.
