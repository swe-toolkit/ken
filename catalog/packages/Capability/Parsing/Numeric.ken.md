# `Capability.Parsing.Numeric` — located decimal parsing

`Capability.Parsing.Numeric` parses decimal characters into arbitrary-precision `Int` values
and reports the exact character index of the first failure. Its formatting
floor is structural: decimal digits convert to characters and back without
crossing the opaque `String`/`List Char` conversion-and-retraction boundary.

## Contents

1. [Motivation](#1-motivation)
2. [Located errors](#2-located-errors)
3. [Decimal parsing](#3-decimal-parsing)
4. [Structural formatting](#4-structural-formatting)
5. [Checked examples](#5-checked-examples)
6. [Trust and derivation](#6-trust-and-derivation)

## 1. Motivation

Parsing is total: success is `Ok`, and failure is an ordinary `Err` value.
Locations count Unicode scalar values in the input `List Char`, never UTF-8
bytes. `Int` is arbitrary-precision, so there is deliberately no overflow
case.

## 2. Located diagnostics

Numeric failures now use the shared `Diagnostic` carrier directly. The caller
supplies a position-to-origin injection, so parsing preserves the exact
character index without inventing a source or argument identity. The two
numeric kinds map to stable diagnostic codes owned by this client package.

```ken
import Capability.Diagnostics.Core
  (ArgumentOrigin,
    Diagnostic,
    DiagnosticCode,
    MkByteRange,
    MkDiagnostic,
    MkDiagnosticCode,
    Origin,
    origin_argument_index,
    origin_range_end,
    origin_range_start)

import Core.Logic.Transport (cong, trans)

data NumericErrorKind = EmptyInput | InvalidDigit

fn numeric_error_code (kind : NumericErrorKind) : DiagnosticCode =
  match kind {
    EmptyInput ↦ MkDiagnosticCode "text.numeric.empty-input";
    InvalidDigit ↦ MkDiagnosticCode "text.numeric.invalid-digit"
  }

fn numeric_diagnostic
      (locate : Nat → Origin) (kind : NumericErrorKind) (position : Nat)
    : Diagnostic =
  MkDiagnostic (locate position) (numeric_error_code kind)

pub fn numeric_argument_origin (argument : Nat) (position : Nat) : Origin =
  ArgumentOrigin argument (MkByteRange position position)

theorem numeric_argument_origin_index_faithful
      (argument : Nat) (position : Nat)
    : Equal
        (Option Nat)
        (origin_argument_index (numeric_argument_origin argument position))
        (Some Nat argument) =
  Refl

theorem numeric_argument_origin_start_faithful
      (argument : Nat) (position : Nat)
    : Equal
        (Option Nat)
        (origin_range_start (numeric_argument_origin argument position))
        (Some Nat position) =
  Refl

theorem numeric_argument_origin_end_faithful
      (argument : Nat) (position : Nat)
    : Equal
        (Option Nat)
        (origin_range_end (numeric_argument_origin argument position))
        (Some Nat position) =
  Refl
```

## 3. Decimal parsing

`char_to_digit` uses the landed identity projection `charToInt` and the landed
integer order. The recursive worker carries both the character index and the
base-ten accumulator.

```ken
pub fn char_to_digit (c : Char) : Option Int =
  match and_bool (leq_int (48 : Int) (charToInt c)) (leq_int (charToInt c) (57 : Int)) {
    True ↦ Some Int (sub_int (charToInt c) (48 : Int));
    False ↦ None Int
  }

pub fn parse_digits_at
      (locate : Nat → Origin) (chars : List Char) (position : Nat) (accumulator : Int)
    : Result Diagnostic Int =
  match chars {
    Nil ↦ Ok Diagnostic Int accumulator;
    Cons c rest ↦
      match char_to_digit c {
        None ↦ Err Diagnostic Int (numeric_diagnostic locate InvalidDigit position);
        Some digit ↦
          parse_digits_at
            locate
            rest
            (Suc position)
            (add_int (mul_int accumulator (10 : Int)) digit)
      }
  }

pub fn parse_nat_chars (locate : Nat → Origin) (chars : List Char) : Result Diagnostic Int =
  match chars {
    Nil ↦ Err Diagnostic Int (numeric_diagnostic locate EmptyInput Zero);
    Cons c rest ↦ parse_digits_at locate (Cons Char c rest) Zero (0 : Int)
  }

fn negate_parsed (x : Result Diagnostic Int) : Result Diagnostic Int =
  match x {
    Err problem ↦ Err Diagnostic Int problem;
    Ok value ↦ Ok Diagnostic Int (sub_int (0 : Int) value)
  }

pub fn parse_int_chars (locate : Nat → Origin) (chars : List Char) : Result Diagnostic Int =
  match chars {
    Nil ↦ Err Diagnostic Int (numeric_diagnostic locate EmptyInput Zero);
    Cons c rest ↦
      match eq_int (charToInt c) (45 : Int) {
        True ↦
          match rest {
            Nil ↦ Err Diagnostic Int (numeric_diagnostic locate EmptyInput (Suc Zero));
            Cons d more ↦
              negate_parsed (parse_digits_at locate (Cons Char d more) (Suc Zero) (0 : Int))
          };
        False ↦ parse_digits_at locate (Cons Char c rest) Zero (0 : Int)
      }
  }

pub fn parse_nat (locate : Nat → Origin) (text : String) : Result Diagnostic Int =
  parse_nat_chars locate (string_to_list_char text)

pub fn parse_int (locate : Nat → Origin) (text : String) : Result Diagnostic Int =
  parse_int_chars locate (string_to_list_char text)
```

### Located parsing laws

The worker equations expose its exact position transition without assigning a
numeric meaning to the opaque accumulator. Entry-point equations distinguish
empty input, a bare sign, signed digits, and unsigned digits. Primitive guards
enter these laws only through their observed Boolean or `Option` values.

```ken
const numeric_zero_accumulator : Int = 0

const numeric_decimal_base : Int = 10

const numeric_minus_code : Int = 45

pub proof empty for parse_digits_at
      (locate : Nat → Origin) (position : Nat) (accumulator : Int)
    : Equal
        (Result Diagnostic Int)
        (parse_digits_at locate (Nil Char) position accumulator)
        (Ok Diagnostic Int accumulator) =
  Refl

pub proof invalid_digit for parse_digits_at
      (locate : Nat → Origin)
      (c : Char)
      (rest : List Char)
      (position : Nat)
      (accumulator : Int)
      (refused : Equal (Option Int) (char_to_digit c) (None Int))
    : Equal
        (Result Diagnostic Int)
        (parse_digits_at locate (Cons Char c rest) position accumulator)
        (Err Diagnostic Int (numeric_diagnostic locate InvalidDigit position)) =
  J
    (λchoice _.
      Equal
        (Result Diagnostic Int)
        (parse_digits_at locate (Cons Char c rest) position accumulator)
        (match choice {
          None ↦ Err Diagnostic Int (numeric_diagnostic locate InvalidDigit position);
          Some digit ↦
            parse_digits_at
              locate
              rest
              (Suc position)
              (add_int (mul_int accumulator numeric_decimal_base) digit)
        }))
    Refl
    refused

pub proof accepted_digit for parse_digits_at
      (locate : Nat → Origin)
      (c : Char)
      (rest : List Char)
      (position : Nat)
      (accumulator : Int)
      (digit : Int)
      (accepted : Equal (Option Int) (char_to_digit c) (Some Int digit))
    : Equal
        (Result Diagnostic Int)
        (parse_digits_at locate (Cons Char c rest) position accumulator)
        (parse_digits_at
          locate
          rest
          (Suc position)
          (add_int (mul_int accumulator numeric_decimal_base) digit)) =
  J
    (λchoice _.
      Equal
        (Result Diagnostic Int)
        (parse_digits_at locate (Cons Char c rest) position accumulator)
        (match choice {
          None ↦ Err Diagnostic Int (numeric_diagnostic locate InvalidDigit position);
          Some next_digit ↦
            parse_digits_at
              locate
              rest
              (Suc position)
              (add_int (mul_int accumulator numeric_decimal_base) next_digit)
        }))
    Refl
    accepted

pub proof empty for parse_nat_chars
      (locate : Nat → Origin)
    : Equal
        (Result Diagnostic Int)
        (parse_nat_chars locate (Nil Char))
        (Err Diagnostic Int (numeric_diagnostic locate EmptyInput Zero)) =
  Refl

pub proof nonempty for parse_nat_chars
      (locate : Nat → Origin) (c : Char) (rest : List Char)
    : Equal
        (Result Diagnostic Int)
        (parse_nat_chars locate (Cons Char c rest))
        (parse_digits_at locate (Cons Char c rest) Zero numeric_zero_accumulator) =
  Refl

pub proof empty for parse_int_chars
      (locate : Nat → Origin)
    : Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Nil Char))
        (Err Diagnostic Int (numeric_diagnostic locate EmptyInput Zero)) =
  Refl

pub proof bare_sign for parse_int_chars
      (locate : Nat → Origin)
      (sign : Char)
      (is_sign : Equal Bool (eq_int (charToInt sign) numeric_minus_code) True)
    : Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char sign (Nil Char)))
        (Err Diagnostic Int (numeric_diagnostic locate EmptyInput (Suc Zero))) =
  J
    (λchoice _.
      Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char sign (Nil Char)))
        (match choice {
          True ↦ Err Diagnostic Int (numeric_diagnostic locate EmptyInput (Suc Zero));
          False ↦
            parse_digits_at locate (Cons Char sign (Nil Char)) Zero numeric_zero_accumulator
        }))
    Refl
    is_sign

pub proof signed for parse_int_chars
      (locate : Nat → Origin)
      (sign : Char)
      (first : Char)
      (rest : List Char)
      (is_sign : Equal Bool (eq_int (charToInt sign) numeric_minus_code) True)
    : Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char sign (Cons Char first rest)))
        (negate_parsed
          (parse_digits_at locate (Cons Char first rest) (Suc Zero) numeric_zero_accumulator)) =
  J
    (λchoice _.
      Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char sign (Cons Char first rest)))
        (match choice {
          True ↦
            negate_parsed
              (parse_digits_at
                locate
                (Cons Char first rest)
                (Suc Zero)
                numeric_zero_accumulator);
          False ↦
            parse_digits_at
              locate
              (Cons Char sign (Cons Char first rest))
              Zero
              numeric_zero_accumulator
        }))
    Refl
    is_sign

pub proof unsigned for parse_int_chars
      (locate : Nat → Origin)
      (first : Char)
      (rest : List Char)
      (not_sign : Equal Bool (eq_int (charToInt first) numeric_minus_code) False)
    : Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char first rest))
        (parse_digits_at locate (Cons Char first rest) Zero numeric_zero_accumulator) =
  J
    (λchoice _.
      Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char first rest))
        (match choice {
          True ↦
            match rest {
              Nil ↦ Err Diagnostic Int (numeric_diagnostic locate EmptyInput (Suc Zero));
              Cons next more ↦
                negate_parsed
                  (parse_digits_at
                    locate
                    (Cons Char next more)
                    (Suc Zero)
                    numeric_zero_accumulator)
            };
          False ↦ parse_digits_at locate (Cons Char first rest) Zero numeric_zero_accumulator
        }))
    Refl
    not_sign
```

The private `numeric_chars_all_accepted` predicate follows the input spine and
requires every `char_to_digit` result to take its accepted branch.
`numeric_parse_succeeded` observes only whether a result is `Ok`. The attached
success proof inducts on the list and never states which opaque `Int` value the
worker returns.

```ken
fn numeric_digit_choice_accepted (choice : Option Int) (tail_accepted : Bool) : Bool =
  match choice {
    None ↦ False;
    Some digit ↦ tail_accepted
  }

fn numeric_chars_all_accepted (chars : List Char) : Bool =
  match chars {
    Nil ↦ True;
    Cons c rest ↦
      numeric_digit_choice_accepted (char_to_digit c) (numeric_chars_all_accepted rest)
  }

fn numeric_parse_succeeded (outcome : Result Diagnostic Int) : Prop =
  match outcome {
    Err problem ↦ Bottom;
    Ok value ↦ Top
  }

theorem numeric_all_accepted_choice
      (choice : Option Int) (tail_accepted : Bool)
    : (goal : Prop)
      → Equal Bool (numeric_digit_choice_accepted choice tail_accepted) True
      → ((digit : Int)
          → Equal
          (Option Int)
          choice
          (Some Int digit)
          → Equal
          Bool
          tail_accepted
          True
          → goal)
      → goal =
  match choice {
    None ↦ λgoal. λaccepted. λrecover. absurd accepted;
    Some digit ↦ λgoal. λaccepted. λrecover. recover digit Refl accepted
  }

proof all_accepted for parse_digits_at
      (locate : Nat → Origin) (chars : List Char) (position : Nat) (accumulator : Int)
    : Equal Bool (numeric_chars_all_accepted chars) True
      → numeric_parse_succeeded (parse_digits_at locate chars position accumulator) =
  match chars {
    Nil ↦ λaccepted. Proved;
    Cons c rest ↦
      λaccepted.
        numeric_all_accepted_choice
          (char_to_digit c)
          (numeric_chars_all_accepted rest)
          (numeric_parse_succeeded
            (parse_digits_at locate (Cons Char c rest) position accumulator))
          accepted
          (λdigit.
            λhead_accepted.
              λtail_accepted.
                J
                  (λrecursive _.
                    numeric_parse_succeeded recursive
                    → numeric_parse_succeeded
                      (parse_digits_at locate (Cons Char c rest) position accumulator))
                  (λsuccess. success)
                  ((proof accepted_digit for parse_digits_at)
                    locate
                    c
                    rest
                    position
                    accumulator
                    digit
                    head_accepted)
                  ((proof all_accepted for parse_digits_at)
                    locate
                    rest
                    (Suc position)
                    (add_int (mul_int accumulator numeric_decimal_base) digit)
                    tail_accepted))
  }
```

Three private witnesses keep the refusal branches distinct. The natural-number
entry point rejects at `Zero`; a bare sign rejects at `Suc Zero`; and an
accepted leading digit followed by a refused character reports `InvalidDigit`
at `Suc Zero`.

```ken
proof empty_refusal for parse_nat_chars
      (locate : Nat → Origin)
    : Equal
        (Result Diagnostic Int)
        (parse_nat_chars locate (Nil Char))
        (Err Diagnostic Int (numeric_diagnostic locate EmptyInput Zero)) =
  (proof empty for parse_nat_chars) locate

proof bare_sign_refusal for parse_int_chars
      (locate : Nat → Origin)
      (sign : Char)
      (is_sign : Equal Bool (eq_int (charToInt sign) numeric_minus_code) True)
    : Equal
        (Result Diagnostic Int)
        (parse_int_chars locate (Cons Char sign (Nil Char)))
        (Err Diagnostic Int (numeric_diagnostic locate EmptyInput (Suc Zero))) =
  (proof bare_sign for parse_int_chars) locate sign is_sign

proof nonzero_invalid_digit_refusal for parse_nat_chars
      (locate : Nat → Origin)
      (first : Char)
      (bad : Char)
      (digit : Int)
      (first_accepted : Equal (Option Int) (char_to_digit first) (Some Int digit))
      (bad_refused : Equal (Option Int) (char_to_digit bad) (None Int))
    : Equal
        (Result Diagnostic Int)
        (parse_nat_chars locate (Cons Char first (Cons Char bad (Nil Char))))
        (Err Diagnostic Int (numeric_diagnostic locate InvalidDigit (Suc Zero))) =
  trans
    (Result Diagnostic Int)
    (parse_nat_chars locate (Cons Char first (Cons Char bad (Nil Char))))
    (parse_digits_at
      locate
      (Cons Char first (Cons Char bad (Nil Char)))
      Zero
      numeric_zero_accumulator)
    (Err Diagnostic Int (numeric_diagnostic locate InvalidDigit (Suc Zero)))
    ((proof nonempty for parse_nat_chars) locate first (Cons Char bad (Nil Char)))
    (trans
      (Result Diagnostic Int)
      (parse_digits_at
        locate
        (Cons Char first (Cons Char bad (Nil Char)))
        Zero
        numeric_zero_accumulator)
      (parse_digits_at
        locate
        (Cons Char bad (Nil Char))
        (Suc Zero)
        (add_int (mul_int numeric_zero_accumulator numeric_decimal_base) digit))
      (Err Diagnostic Int (numeric_diagnostic locate InvalidDigit (Suc Zero)))
      ((proof accepted_digit for parse_digits_at)
        locate
        first
        (Cons Char bad (Nil Char))
        Zero
        numeric_zero_accumulator
        digit
        first_accepted)
      ((proof invalid_digit for parse_digits_at)
        locate
        bad
        (Nil Char)
        (Suc Zero)
        (add_int (mul_int numeric_zero_accumulator numeric_decimal_base) digit)
        bad_refused))
```

## 4. Structural formatting

The format direction uses an explicit digit carrier. This makes its verified
round trip a purely structural `List DecimalDigit`/`List Char` theorem. The
String-facing wrapper remains only a function; no universal String bijection
law is asserted. A total `show_int : Int → String` is a named fast-follow:
opaque `Int` has no division, remainder, destructor, or `Int → Nat` bridge, so
CC2 does not fake that missing operation with a bounded table or a
non-structural loop.

```ken
data DecimalDigit : Type where {
  MkDecimalDigit :
    (value : Int)
    → (glyph : Char)
    → Equal (Option Int) (char_to_digit glyph) (Some Int value)
    → DecimalDigit
}

fn decimal_digit_value (digit : DecimalDigit) : Int =
  match digit {
    MkDecimalDigit value glyph valid ↦ value
  }

fn decimal_digit_to_char (digit : DecimalDigit) : Char =
  match digit {
    MkDecimalDigit value glyph valid ↦ glyph
  }

proof valid for decimal_digit_to_char
      (digit : DecimalDigit)
    : Equal
        (Option Int)
        (char_to_digit (decimal_digit_to_char digit))
        (Some Int (decimal_digit_value digit)) =
  match digit {
    MkDecimalDigit value glyph valid ↦ valid
  }

fn decimal_digit_values (digits : List DecimalDigit) : List Int =
  match digits {
    Nil ↦ Nil Int;
    Cons digit rest ↦ Cons Int (decimal_digit_value digit) (decimal_digit_values rest)
  }

fn format_digits (digits : List DecimalDigit) : List Char =
  match digits {
    Nil ↦ Nil Char;
    Cons digit rest ↦ Cons Char (decimal_digit_to_char digit) (format_digits rest)
  }

fn parsed_int_prepend (digit : Int) (parsed : Option (List Int)) : Option (List Int) =
  match parsed {
    None ↦ None (List Int);
    Some rest ↦ Some (List Int) (Cons Int digit rest)
  }

fn parse_digit_result
      (parsed_rest : Option (List Int)) (parsed_digit : Option Int)
    : Option (List Int) =
  match parsed_digit {
    None ↦ None (List Int);
    Some digit ↦ parsed_int_prepend digit parsed_rest
  }

pub fn parse_formatted_digits (chars : List Char) : Option (List Int) =
  match chars {
    Nil ↦ Some (List Int) (Nil Int);
    Cons c rest ↦ parse_digit_result (parse_formatted_digits rest) (char_to_digit c)
  }

fn show_digits (digits : List DecimalDigit) : String =
  list_char_to_string (format_digits digits)

theorem format_digits_roundtrip
      (digits : List DecimalDigit)
    : Equal
        (Option (List Int))
        (parse_formatted_digits (format_digits digits))
        (Some (List Int) (decimal_digit_values digits)) =
  match digits {
    Nil ↦ Proved;
    Cons digit rest ↦
      trans
        (Option (List Int))
        (parse_digit_result
          (parse_formatted_digits (format_digits rest))
          (char_to_digit (decimal_digit_to_char digit)))
        (parsed_int_prepend
          (decimal_digit_value digit)
          (parse_formatted_digits (format_digits rest)))
        (Some (List Int) (Cons Int (decimal_digit_value digit) (decimal_digit_values rest)))
        (cong
          (Option Int)
          (Option (List Int))
          (char_to_digit (decimal_digit_to_char digit))
          (Some Int (decimal_digit_value digit))
          (parse_digit_result (parse_formatted_digits (format_digits rest)))
          ((proof valid for decimal_digit_to_char) digit))
        (cong
          (Option (List Int))
          (Option (List Int))
          (parse_formatted_digits (format_digits rest))
          (Some (List Int) (decimal_digit_values rest))
          (parsed_int_prepend (decimal_digit_value digit))
          (format_digits_roundtrip rest))
  }
```

## 5. Checked examples

These cases pin the valid, empty, negative, and exact-index failure semantics.

```ken example
const digit_zero_result : Option Int = char_to_digit (48 : Int)

const digit_nine_result : Option Int = char_to_digit (57 : Int)

const letter_digit_result : Option Int = char_to_digit (120 : Int)

fn example_numeric_origin (position : Nat) : Origin =
  numeric_argument_origin (Suc (Suc Zero)) position

const parsed_decimal_result : Result Diagnostic Int = parse_nat example_numeric_origin "123"

const empty_input_result : Result Diagnostic Int = parse_nat example_numeric_origin ""

const bad_digit_result : Result Diagnostic Int = parse_nat example_numeric_origin "12x4"

const parsed_negative_result : Result Diagnostic Int = parse_int example_numeric_origin "-42"
```

## 6. Trust and derivation

**Public API:** `numeric_argument_origin`, `char_to_digit`, `parse_digits_at`,
its attached `empty`, `invalid_digit`, and `accepted_digit` laws,
`parse_nat_chars`, its attached `empty` and `nonempty` laws, `parse_int_chars`,
its attached `empty`, `bare_sign`, `signed`, and `unsigned` laws, `parse_nat`,
`parse_int`, and `parse_formatted_digits`.

**Derivation.** Parsing uses structural recursion on `List Char`, positions use
structural `Nat`, and values use the landed `charToInt`, `leq_int`, `eq_int`,
`add_int`, `mul_int`, and `sub_int` operations. The verified format law never
mentions `String` or consumes an opaque `Int`; `show_digits` is only a function
across the String boundary. `show_int : Int → String` remains deferred until a
sound Int-destruction substrate exists.

**Proof families.** Parsing dispatch laws split the same structural or
primitive-value guards as their subjects. The success characterization is a
structural induction on `List Char`; its recursive appeal advances the position
by one and preserves the accumulator expression without interpreting it.

**Trust delta:** zero. The package declares no primitive, postulate, opaque
constant, or `Axiom`.
