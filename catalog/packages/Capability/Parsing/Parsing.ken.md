# `parsing` — source artifacts, spans, parsers, and a Boolean grammar

The source/span core, a total parser-result surface, and a fully
parenthesized Boolean-expression grammar built end to end on top of it. This
package models source identity as an immutable byte artifact: spans are only
half-open byte endpoints, and source identity is supplied by values such as
`Located` and by validity predicates, never by a bare `Span`.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws  proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust  derivation](#7-trust--derivation)

## 1. Motivation

Every parser needs vocabulary to state where in the source a value came from
and
whether a parse succeeded: a `SourceId` to disambiguate which source, `Bytes`
(not codepoints) as the offset basis for a `Span`, and a `ParseResult` that
is total — `Parsed` or `Failed`, never a partial function — with public
validity predicates a caller can check rather than trust. This package
states that vocabulary once, then exercises the whole stack end to end with
one concrete parser: a fully parenthesized Boolean-expression grammar.

## 2. Definition

`SourceId` comes from the lower `Capability.Diagnostics.Core` package. `Source` is a
checked record carrying artifact identity, the original `Bytes`, and UTF-8
evidence. Lengths and positions are structural `Nat` quantities computed from
the total `List UInt8` view. `String` is deliberately not the offset basis;
`IsUtf8` is a proof the bytes *happen* to decode losslessly, not a requirement
they must.

```ken
import Capability.Diagnostics.Core
  (ByteRange,
    MkByteRange,
    Origin,
    SourceId,
    SourceOrigin,
    byte_range_end,
    byte_range_start,
    origin_source_id)

import Capability.Parsing.Cursor
  (CursorOps, MkCursorOps, cursor_advance, cursor_locate, cursor_peek)

import Capability.Parsing.Decoder
  (Decoded,
    Decoder,
    DecoderError,
    DecoderFailed,
    DecoderPreserves,
    DecoderRejected,
    DecoderResult,
    decoder_alt,
    decoder_alt_preserves,
    decoder_error_location,
    decoder_fail,
    decoder_many,
    decoder_many_preserves,
    decoder_pure,
    decoder_recursive,
    decoder_recursive_preserves,
    decoder_satisfy,
    decoder_satisfy_preserves,
    decoder_seq,
    decoder_seq_preserves)

import Core.Classes.LawfulClasses (leq_nat)

import Data.Collections.Derived (bytes_nat_length, list_append, nth)

import Data.Numeric.Nat.Order (sub)

pub fn IsUtf8 (bs : Bytes) : Prop =
  match bytes_decode bs {
    Err _ ↦ Bottom;
    Ok text ↦ Equal Bytes (bytes_encode text) bs
  }

pub class Source {
  source_id_field : SourceId;
  source_bytes_field : Bytes;
  source_utf8_field : IsUtf8 source_bytes_field
}

pub fn source_id (s : Source) : SourceId = s.source_id_field

pub fn source_bytes (s : Source) : Bytes = s.source_bytes_field

pub fn source_length (s : Source) : Nat = bytes_nat_length s.source_bytes_field

pub proof utf8 for source_bytes (s : Source) : IsUtf8 (source_bytes s) = s.source_utf8_field
```

## 3. Using it

A caller builds a `Source` once per artifact (supplying its artifact identity,
bytes, and `source_bytes::utf8` evidence; length is computed), then drives
`§4.3`'s `parse_bool_expr : Parser (Syntax BoolExpr)` over it. `format_bool_expr`
is the single-call entry point: it parses a `Source` end to end and, on
success, prints the erased (span-free) tree back out — the worked round
trip this package ships as its concrete example, in `§4.3` rather than a
separate illustrative fence, since it is real, checked package content, not
a demo.

## 4. Laws  proofs

### 4.1 Span validity and the zero-width-span proof

`Span` carries only half-open byte endpoints — a bare `Span` never
identifies a source artifact by itself. `ValidSpan s sp` requires
`span_start sp <= span_end sp <= source_length s`, stated through
`LessEqNat`, the same `Bool`-bridged pattern the lawful-classes packages use
(`Equal Bool (leq_nat m n) True`), here without a named `IsTrue` alias
since this package has no `Eq`/`Ord`-style class to hang one on.
`LessEqNat::refl` is a genuine proof by induction on `n`; `LessEqNat::zero_left`
is definitional (`leq_nat Zero n` reduces to `True` on its very first
match arm, for any `n`). `valid_zero_width_span` is the one composite proof
in this package: given a valid offset (`LessEqNat offset (source_length s)`),
a zero-width span at that offset is valid, by pairing `LessEqNat::refl`
(the span's own `start <= end`, both `offset`) with the supplied hypothesis
(`end <= source_length s`) via `and_intro`.

```ken
export Span, MkSpan

data Span = MkSpan Nat Nat

pub fn span_start (sp : Span) : Nat =
  match sp {
    MkSpan start end ↦ start
  }

pub fn span_end (sp : Span) : Nat =
  match sp {
    MkSpan start end ↦ end
  }

pub fn span_to_byte_range (sp : Span) : ByteRange = MkByteRange (span_start sp) (span_end sp)

pub fn span_origin (source : SourceId) (sp : Span) : Origin =
  SourceOrigin source (span_to_byte_range sp)

theorem span_to_byte_range_faithful
      (sp : Span)
    : And
        (Equal Nat (byte_range_start (span_to_byte_range sp)) (span_start sp))
        (Equal Nat (byte_range_end (span_to_byte_range sp)) (span_end sp)) =
  match sp {
    MkSpan start end ↦ and_intro (Equal Nat start start) (Equal Nat end end) Refl Refl
  }

theorem span_origin_source_faithful
      (source : SourceId) (sp : Span)
    : Equal
        (Option SourceId)
        (origin_source_id (span_origin source sp))
        (Some SourceId source) =
  match sp {
    MkSpan start end ↦ Refl
  }

export ByteCursor

data ByteCursor = MkByteCursor Source Nat

fn byte_cursor_source (cur : ByteCursor) : Source =
  match cur {
    MkByteCursor source position ↦ source
  }

fn byte_cursor_position (cur : ByteCursor) : Nat =
  match cur {
    MkByteCursor source position ↦ position
  }

fn byte_cursor_remaining (cur : ByteCursor) : Nat =
  sub (source_length (byte_cursor_source cur)) (byte_cursor_position cur)

fn byte_cursor_peek (cur : ByteCursor) : Option UInt8 =
  nth UInt8 (byte_cursor_position cur) (bytes_to_list (source_bytes (byte_cursor_source cur)))

fn byte_cursor_advance (cur : ByteCursor) : ByteCursor =
  MkByteCursor (byte_cursor_source cur) (Suc (byte_cursor_position cur))

fn byte_cursor_locate (cur : ByteCursor) : Span =
  MkSpan (byte_cursor_position cur) (byte_cursor_position cur)

pub const byte_cursor_ops : CursorOps ByteCursor UInt8 Span =
  MkCursorOps
    ByteCursor
    UInt8
    Span
    byte_cursor_remaining
    byte_cursor_peek
    byte_cursor_advance
    byte_cursor_locate

pub fn LessEqNat (m : Nat) (n : Nat) : Prop = Equal Bool (leq_nat m n) True

pub proof refl for LessEqNat (n : Nat) : LessEqNat n n =
  match n {
    Zero ↦ Proved;
    Suc n2 ↦ proof refl for LessEqNat n2
  }

pub proof zero_left for LessEqNat (n : Nat) : LessEqNat Zero n = Proved

pub fn ValidSpan (s : Source) (sp : Span) : Prop =
  And (LessEqNat (span_start sp) (span_end sp)) (LessEqNat (span_end sp) (source_length s))

pub theorem valid_zero_width_span
      (s : Source) (offset : Nat)
    : LessEqNat offset (source_length s) → ValidSpan s (MkSpan offset offset) =
  λh.
    and_intro
      (LessEqNat offset offset)
      (LessEqNat offset (source_length s))
      ((proof refl for LessEqNat) offset)
      h
```

### 4.2 Located values, parse errors, and the total `Parser` contract

`Located a` pairs a value with a `SourceId` and `Span`; `ValidLocated`
checks both the source id and span against a concrete `Source`. `ParseError`
carries just enough to be checkable the same way — a `SourceId` and `Span`
of its own. `Parser a` is total *by construction*: it always returns a
`ParseResult a` (`Parsed` or `Failed`), never diverges or partial-applies,
conditional on the caller supplying a proof the start position is in bounds
(`LessEqNat start (source_length s)`). `ParserValid`/`ParserTotal`/
`ParserSourceLocal` are the three public well-formedness *properties* a
`Parser` should satisfy — plain predicates over a `Parser a`, not enforced
by the `Parser` type itself, so a caller can state and check them per
concrete parser. `parser_pure` and `parser_fail` are the two base combinators:
the former always succeeds on a zero-width span at `start`, the latter
always fails at `start` with a zero-width error span. The Boolean parser's
`parse_bool_expr_total` inhabits `ParserTotal` alone; the attached
`parse_bool_expr_laws` proof also establishes `ParserValid` and
`ParserSourceLocal` from the Decoder preservation laws and the checked
source-bound cursor invariant.

```ken
export Located, MkLocated

data Located a = MkLocated SourceId Span a

pub fn located_source (a : Type) (x : Located a) : SourceId =
  match x {
    MkLocated sid sp value ↦ sid
  }

pub fn located_span (a : Type) (x : Located a) : Span =
  match x {
    MkLocated sid sp value ↦ sp
  }

pub fn located_value (a : Type) (x : Located a) : a =
  match x {
    MkLocated sid sp value ↦ value
  }

pub fn ValidLocated (a : Type) (s : Source) (x : Located a) : Prop =
  And (Equal SourceId (located_source a x) (source_id s)) (ValidSpan s (located_span a x))

export ParseError, MkParseError

data ParseError = MkParseError SourceId Span

pub fn error_source (err : ParseError) : SourceId =
  match err {
    MkParseError sid sp ↦ sid
  }

pub fn error_span (err : ParseError) : Span =
  match err {
    MkParseError sid sp ↦ sp
  }

export ParseResult, Parsed, Failed

data ParseResult a = Parsed a Span Nat | Failed ParseError

pub const Parser (a : Type) : Type =
  (s : Source) → (start : Nat) → LessEqNat start (source_length s) → ParseResult a

fn decoder_parse_error (s : Source) (err : DecoderError Span) : ParseError =
  MkParseError (source_id s) (decoder_error_location Span err)

pub fn parser_from_decoder (a : Type) (decoder : Decoder ByteCursor Span a) : Parser a =
  λs.
    λstart.
      λh.
        match decoder (MkByteCursor s start) {
          Decoded value next ↦
            Parsed
              a
              value
              (MkSpan start (byte_cursor_position next))
              (byte_cursor_position next);
          DecoderFailed err ↦ Failed a (decoder_parse_error s err)
        }

pub fn ParsedValid (s : Source) (start : Nat) (consumed : Span) (next : Nat) : Prop =
  And
    (ValidSpan s consumed)
    (And (Equal Nat (span_start consumed) start) (Equal Nat (span_end consumed) next))

pub fn FailedValid (s : Source) (err : ParseError) : Prop =
  And (Equal SourceId (error_source err) (source_id s)) (ValidSpan s (error_span err))

pub fn ParseResultValid (a : Type) (s : Source) (start : Nat) (r : ParseResult a) : Prop =
  match r {
    Parsed value consumed next ↦ ParsedValid s start consumed next;
    Failed err ↦ FailedValid s err
  }

pub fn ParserValid (a : Type) (p : Parser a) : Prop =
  (s : Source)
    → (start : Nat)
    → (h : LessEqNat start (source_length s))
    → ParseResultValid a s start
    (p s start h)

fn ParseResultTotal (a : Type) (r : ParseResult a) : Prop =
  match r {
    Parsed value consumed next ↦ Top;
    Failed err ↦ Top
  }

pub fn ParserTotal (a : Type) (p : Parser a) : Prop =
  (s : Source)
    → (start : Nat)
    → (h : LessEqNat start (source_length s))
    → ParseResultTotal a
    (p s start h)

fn ParseResultSourceLocal (a : Type) (s : Source) (r : ParseResult a) : Prop =
  match r {
    Parsed value consumed next ↦ ValidSpan s consumed;
    Failed err ↦ Equal SourceId (error_source err) (source_id s)
  }

pub fn ParserSourceLocal (a : Type) (p : Parser a) : Prop =
  (s : Source)
    → (start : Nat)
    → (h : LessEqNat start (source_length s))
    → ParseResultSourceLocal a s
    (p s start h)

pub fn ParserLaws (a : Type) (p : Parser a) : Prop =
  And (ParserValid a p) (And (ParserTotal a p) (ParserSourceLocal a p))

pub fn parser_pure (a : Type) (value : a) : Parser a =
  parser_from_decoder a (decoder_pure ByteCursor Span a value)

pub const parser_fail (a : Type) : Parser a =
  parser_from_decoder a (decoder_fail ByteCursor UInt8 Span a byte_cursor_ops)
```

### 4.3 A worked grammar: parenthesized Boolean expressions

`BoolExpr` is fully parenthesized: `true`, `false`, `(not e)`, and
`(and e1 e2)`. There is no precedence table — `true and false` rejects,
deliberately; a real expression grammar with precedence is out of scope for
this worked example. `Syntax a` pairs a `Located a` root with a `List` of
`Located a` children, giving every parsed node its own span independent of
its value's own recursive structure; `erase_spans` recovers the bare
`BoolExpr` by walking back down to the root value.

Token recognition is byte-by-byte through CAT-5's explicit
`byte_cursor_ops`, matching literal ASCII codepoints spelled as `Int` literals
(`116` is `t`, `102` is `f`, `40` is `(`, `32` is space, and so on). The
worked grammar is a genuine `Capability.Parsing.Decoder` client: fixed tokens use
`decoder_satisfy`/`decoder_seq`, whitespace uses progress-checked
`decoder_many`, and recursive expressions use `decoder_recursive`. Both
repetition and recursive descent seed their private structural fuel from the
cursor's `remaining`; the old CAT-5-local fuel recursions are retired.
The grammar selectively imports `list_append` from
`Data.Collections.Derived` to assemble child lists without maintaining a
package-local copy. `format_bool_expr_on_parse_success` proves that a
successful parse is formatted by printing its span-erased result;
`format_bool_expr_on_parse_failure` preserves its parse error. The
success premise is an actual parse, not an assumption that arbitrary
printer output will parse.

The private bounds bridge proves that successful byte tokens remain within
the original source's structural byte length. Public Decoder preservation
laws carry the same bound through token sequencing, whitespace repetition,
alternatives, and recursive grammar layers. The checked local continuation
lemmas preserve it through each syntax-building branch and the final
end-of-input check. `parse_bool_expr_laws` applies that result to the
unweakened `ParserValid`, `ParserTotal`, and `ParserSourceLocal` contract.
The independent printer-to-parser round trip is not claimed: it still needs
primitive `Bytes` concatenation-view and encoded-literal facts.

```ken
export BoolExpr, BTrue, BFalse, BNot, BAnd

data BoolExpr = BTrue | BFalse | BNot BoolExpr | BAnd BoolExpr BoolExpr

export Syntax, MkSyntax

data Syntax a = MkSyntax (Located a) (List (Located a))

pub fn syntax_root (a : Type) (x : Syntax a) : Located a =
  match x {
    MkSyntax root children ↦ root
  }

pub fn syntax_children (a : Type) (x : Syntax a) : List (Located a) =
  match x {
    MkSyntax root children ↦ children
  }

pub fn erase_spans (x : Syntax BoolExpr) : BoolExpr =
  located_value BoolExpr (syntax_root BoolExpr x)

pub fn ValidLocatedList (a : Type) (s : Source) (xs : List (Located a)) : Prop =
  match xs {
    Nil ↦ Top;
    Cons x rest ↦ And (ValidLocated a s x) (ValidLocatedList a s rest)
  }

pub fn ValidSyntax (a : Type) (s : Source) (x : Syntax a) : Prop =
  And (ValidLocated a s (syntax_root a x)) (ValidLocatedList a s (syntax_children a x))

fn bool_expr_eq (x : BoolExpr) (y : BoolExpr) : Bool =
  match x {
    BTrue ↦
      match y {
        BTrue ↦ True;
        BFalse ↦ False;
        BNot y1 ↦ False;
        BAnd yl yr ↦ False
      };
    BFalse ↦
      match y {
        BTrue ↦ False;
        BFalse ↦ True;
        BNot y1 ↦ False;
        BAnd yl yr ↦ False
      };
    BNot x1 ↦
      match y {
        BTrue ↦ False;
        BFalse ↦ False;
        BNot y1 ↦ bool_expr_eq x1 y1;
        BAnd yl yr ↦ False
      };
    BAnd xl xr ↦
      match y {
        BTrue ↦ False;
        BFalse ↦ False;
        BNot y1 ↦ False;
        BAnd yl yr ↦
          match bool_expr_eq xl yl {
            True ↦ bool_expr_eq xr yr;
            False ↦ False
          }
      }
  }

fn syntax_leaf (s : Source) (start : Nat) (end : Nat) (value : BoolExpr) : Syntax BoolExpr =
  MkSyntax
    BoolExpr
    (MkLocated BoolExpr (source_id s) (MkSpan start end) value)
    (Nil (Located BoolExpr))

fn syntax_node_unary
      (s : Source) (start : Nat) (end : Nat) (value : BoolExpr) (child : Syntax BoolExpr)
    : Syntax BoolExpr =
  MkSyntax
    BoolExpr
    (MkLocated BoolExpr (source_id s) (MkSpan start end) value)
    (Cons (Located BoolExpr) (syntax_root BoolExpr child) (syntax_children BoolExpr child))

fn syntax_node_binary
      (s : Source)
      (start : Nat)
      (end : Nat)
      (value : BoolExpr)
      (left : Syntax BoolExpr)
      (right : Syntax BoolExpr)
    : Syntax BoolExpr =
  MkSyntax
    BoolExpr
    (MkLocated BoolExpr (source_id s) (MkSpan start end) value)
    (list_append
      (Located BoolExpr)
      (Cons (Located BoolExpr) (syntax_root BoolExpr left) (syntax_children BoolExpr left))
      (Cons (Located BoolExpr) (syntax_root BoolExpr right) (syntax_children BoolExpr right)))

fn byte_code_decoder (code : Int) : Decoder ByteCursor Span UInt8 =
  decoder_satisfy ByteCursor UInt8 Span byte_cursor_ops (λbyte. eq_int (uint8_to_int byte) code)

const true_token_decoder : Decoder ByteCursor Span UInt8 =
  decoder_seq
    ByteCursor
    Span
    UInt8
    UInt8
    (byte_code_decoder (116 : Int))
    (decoder_seq
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder (114 : Int))
      (decoder_seq
        ByteCursor
        Span
        UInt8
        UInt8
        (byte_code_decoder (117 : Int))
        (byte_code_decoder (101 : Int))))

const false_token_decoder : Decoder ByteCursor Span UInt8 =
  decoder_seq
    ByteCursor
    Span
    UInt8
    UInt8
    (byte_code_decoder (102 : Int))
    (decoder_seq
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder (97 : Int))
      (decoder_seq
        ByteCursor
        Span
        UInt8
        UInt8
        (byte_code_decoder (108 : Int))
        (decoder_seq
          ByteCursor
          Span
          UInt8
          UInt8
          (byte_code_decoder (115 : Int))
          (byte_code_decoder (101 : Int)))))

const not_open_token_decoder : Decoder ByteCursor Span UInt8 =
  decoder_seq
    ByteCursor
    Span
    UInt8
    UInt8
    (byte_code_decoder (40 : Int))
    (decoder_seq
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder (110 : Int))
      (decoder_seq
        ByteCursor
        Span
        UInt8
        UInt8
        (byte_code_decoder (111 : Int))
        (decoder_seq
          ByteCursor
          Span
          UInt8
          UInt8
          (byte_code_decoder (116 : Int))
          (byte_code_decoder (32 : Int)))))

const and_open_token_decoder : Decoder ByteCursor Span UInt8 =
  decoder_seq
    ByteCursor
    Span
    UInt8
    UInt8
    (byte_code_decoder (40 : Int))
    (decoder_seq
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder (97 : Int))
      (decoder_seq
        ByteCursor
        Span
        UInt8
        UInt8
        (byte_code_decoder (110 : Int))
        (decoder_seq
          ByteCursor
          Span
          UInt8
          UInt8
          (byte_code_decoder (100 : Int))
          (byte_code_decoder (32 : Int)))))

const spaces_decoder : Decoder ByteCursor Span (List UInt8) =
  decoder_many ByteCursor UInt8 Span UInt8 byte_cursor_ops (byte_code_decoder (32 : Int))

fn bool_true_decoder (cur : ByteCursor) : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  match true_token_decoder cur {
    DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
    Decoded ignored next ↦
      Decoded
        ByteCursor
        Span
        (Syntax BoolExpr)
        (syntax_leaf
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position next)
          BTrue)
        next
  }

fn bool_false_decoder (cur : ByteCursor) : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  match false_token_decoder cur {
    DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
    Decoded ignored next ↦
      Decoded
        ByteCursor
        Span
        (Syntax BoolExpr)
        (syntax_leaf
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position next)
          BFalse)
        next
  }

fn bool_not_decoder
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
    : Decoder ByteCursor Span (Syntax BoolExpr) =
  λcur.
    match not_open_token_decoder cur {
      DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
      Decoded ignored after_open ↦
        match spaces_decoder after_open {
          DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
          Decoded spaces child_start ↦
            match recur child_start {
              DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
              Decoded child child_end ↦
                match spaces_decoder child_end {
                  DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                  Decoded trailing close_start ↦
                    match byte_code_decoder (41 : Int) close_start {
                      DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                      Decoded close after_close ↦
                        Decoded
                          ByteCursor
                          Span
                          (Syntax BoolExpr)
                          (syntax_node_unary
                            (byte_cursor_source cur)
                            (byte_cursor_position cur)
                            (byte_cursor_position after_close)
                            (BNot (erase_spans child))
                            child)
                          after_close
                    }
                }
            }
        }
    }

fn bool_and_decoder
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
    : Decoder ByteCursor Span (Syntax BoolExpr) =
  λcur.
    match and_open_token_decoder cur {
      DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
      Decoded ignored after_open ↦
        match spaces_decoder after_open {
          DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
          Decoded leading left_start ↦
            match recur left_start {
              DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
              Decoded left left_end ↦
                match byte_code_decoder (32 : Int) left_end {
                  DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                  Decoded separator after_separator ↦
                    match spaces_decoder after_separator {
                      DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                      Decoded middle right_start ↦
                        match recur right_start {
                          DecoderFailed err ↦
                            DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                          Decoded right right_end ↦
                            match spaces_decoder right_end {
                              DecoderFailed err ↦
                                DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                              Decoded trailing close_start ↦
                                match byte_code_decoder (41 : Int) close_start {
                                  DecoderFailed err ↦
                                    DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
                                  Decoded close after_close ↦
                                    Decoded
                                      ByteCursor
                                      Span
                                      (Syntax BoolExpr)
                                      (syntax_node_binary
                                        (byte_cursor_source cur)
                                        (byte_cursor_position cur)
                                        (byte_cursor_position after_close)
                                        (BAnd (erase_spans left) (erase_spans right))
                                        left
                                        right)
                                      after_close
                                }
                            }
                        }
                    }
                }
            }
        }
    }

fn bool_decoder_layer
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
    : Decoder ByteCursor Span (Syntax BoolExpr) =
  decoder_alt
    ByteCursor
    Span
    (Syntax BoolExpr)
    bool_true_decoder
    (decoder_alt
      ByteCursor
      Span
      (Syntax BoolExpr)
      bool_false_decoder
      (decoder_alt
        ByteCursor
        Span
        (Syntax BoolExpr)
        (bool_not_decoder recur)
        (bool_and_decoder recur)))

const bool_expression_decoder : Decoder ByteCursor Span (Syntax BoolExpr) =
  decoder_recursive ByteCursor UInt8 Span (Syntax BoolExpr) byte_cursor_ops bool_decoder_layer

fn complete_bool_decoder (cur : ByteCursor) : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  match spaces_decoder cur {
    DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
    Decoded leading start ↦
      match bool_expression_decoder start {
        DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
        Decoded syntax next ↦
          match spaces_decoder next {
            DecoderFailed err ↦ DecoderFailed ByteCursor Span (Syntax BoolExpr) err;
            Decoded trailing end ↦
              match byte_cursor_remaining end {
                Zero ↦ Decoded ByteCursor Span (Syntax BoolExpr) syntax end;
                Suc rest ↦
                  DecoderFailed
                    ByteCursor
                    Span
                    (Syntax BoolExpr)
                    (DecoderRejected Span (byte_cursor_locate end))
              }
          }
      }
  }

pub const parse_bool_expr : Parser (Syntax BoolExpr) =
  parser_from_decoder (Syntax BoolExpr) complete_bool_decoder

pub fn print_bool_expr (e : BoolExpr) : Bytes =
  match e {
    BTrue ↦ bytes_encode "true";
    BFalse ↦ bytes_encode "false";
    BNot child ↦
      bytes_concat
        (bytes_concat (bytes_encode "(not ") (print_bool_expr child))
        (bytes_encode ")");
    BAnd left right ↦
      bytes_concat
        (bytes_concat
          (bytes_concat (bytes_encode "(and ") (print_bool_expr left))
          (bytes_encode " "))
        (bytes_concat (print_bool_expr right) (bytes_encode ")"))
  }

pub fn format_bool_expr (s : Source) : Result ParseError Bytes =
  match parse_bool_expr s Zero ((proof zero_left for LessEqNat) (source_length s)) {
    Parsed syntax consumed next ↦ Ok ParseError Bytes (print_bool_expr (erase_spans syntax));
    Failed err ↦ Err ParseError Bytes err
  }

fn format_bool_parse_outcome
      (outcome : ParseResult (Syntax BoolExpr))
    : Result ParseError Bytes =
  match outcome {
    Parsed syntax consumed next ↦ Ok ParseError Bytes (print_bool_expr (erase_spans syntax));
    Failed err ↦ Err ParseError Bytes err
  }

pub theorem parse_bool_expr_total : ParserTotal (Syntax BoolExpr) parse_bool_expr =
  λs.
    λstart.
      λh.
        match parse_bool_expr s start h {
          Parsed syntax consumed next ↦ Proved;
          Failed err ↦ Proved
        }

theorem nat_leq_suc (n : Nat) : LessEqNat n (Suc n) =
  match n {
    Zero ↦ Proved;
    Suc previous ↦ nat_leq_suc previous
  }

theorem bytes_refl (b : Bytes) : Equal Bytes b b = Refl

theorem source_length_from_bytes
      (s : Source) (t : Source) (same_bytes : Equal Bytes (source_bytes t) (source_bytes s))
    : Equal Nat (source_length t) (source_length s) =
  J (λbs _. Equal Nat (bytes_nat_length (source_bytes t)) (bytes_nat_length bs)) Refl same_bytes

theorem leq_nat_at_equal_upper
      (lower : Nat)
      (upper : Nat)
      (equal_upper : Nat)
      (same_upper : Equal Nat upper equal_upper)
      (bounded : LessEqNat lower upper)
    : LessEqNat lower equal_upper =
  J (λupper2 _. LessEqNat lower upper2) bounded same_upper

theorem byte_cursor_peek_in_bounds
      (s : Source)
      (position : Nat)
      (value : UInt8)
      (peeked : Equal
        (Option UInt8)
        (byte_cursor_peek (MkByteCursor s position))
        (Some UInt8 value))
    : LessEqNat (Suc position) (source_length s) =
  (proof some_below_length for nth) UInt8 position (bytes_to_list (source_bytes s)) value peeked

theorem byte_cursor_advance_bounded_for_source
      (s : Source)
      (t : Source)
      (start : Nat)
      (position : Nat)
      (value : UInt8)
      (same_bytes : Equal Bytes (source_bytes t) (source_bytes s))
      (start_bounded : LessEqNat start position)
      (peeked : Equal
        (Option UInt8)
        (byte_cursor_peek (MkByteCursor t position))
        (Some UInt8 value))
    : And (LessEqNat start (Suc position)) (LessEqNat (Suc position) (source_length s)) =
  let advanced =
    byte_cursor_advance_bounded t start position value start_bounded peeked
  in
    and_intro
      (LessEqNat start (Suc position))
      (LessEqNat (Suc position) (source_length s))
      (and_fst
        (LessEqNat start (Suc position))
        (LessEqNat (Suc position) (source_length t))
        advanced)
      (leq_nat_at_equal_upper
        (Suc position)
        (source_length t)
        (source_length s)
        (source_length_from_bytes s t same_bytes)
        (and_snd
          (LessEqNat start (Suc position))
          (LessEqNat (Suc position) (source_length t))
          advanced))

theorem byte_cursor_advance_bounded
      (s : Source)
      (start : Nat)
      (position : Nat)
      (value : UInt8)
      (start_bounded : LessEqNat start position)
      (peeked : Equal
        (Option UInt8)
        (byte_cursor_peek (MkByteCursor s position))
        (Some UInt8 value))
    : And (LessEqNat start (Suc position)) (LessEqNat (Suc position) (source_length s)) =
  and_intro
    (LessEqNat start (Suc position))
    (LessEqNat (Suc position) (source_length s))
    ((proof trans for leq_nat)
      start
      position
      (Suc position)
      start_bounded
      (nat_leq_suc position))
    (byte_cursor_peek_in_bounds s position value peeked)

fn DecoderOutcomeBounded
      (a : Type) (s : Source) (start : Nat) (outcome : DecoderResult ByteCursor Span a)
    : Prop =
  match outcome {
    Decoded value next ↦
      And
        (Equal Bytes (source_bytes (byte_cursor_source next)) (source_bytes s))
        (And
          (LessEqNat start (byte_cursor_position next))
          (LessEqNat (byte_cursor_position next) (source_length s)));
    DecoderFailed err ↦ ValidSpan s (decoder_error_location Span err)
  }

fn ByteCursorBounded (s : Source) (start : Nat) (cur : ByteCursor) : Prop =
  And
    (Equal Bytes (source_bytes (byte_cursor_source cur)) (source_bytes s))
    (And
      (LessEqNat start (byte_cursor_position cur))
      (LessEqNat (byte_cursor_position cur) (source_length s)))

fn DecoderPreservesBounded (a : Type) (decoder : Decoder ByteCursor Span a) : Prop =
  (s : Source)
    → (start : Nat)
    → (cur : ByteCursor)
    → ByteCursorBounded s start cur → DecoderOutcomeBounded a s start
    (decoder cur)

theorem option_prop_elim
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

theorem bool_prop_elim
      (motive : Bool → Prop) (value : Bool) (on_true : motive True) (on_false : motive False)
    : motive value =
  match value {
    True ↦ on_true;
    False ↦ on_false
  }

fn byte_code_decoder_accepted
      (s : Source) (position : Nat) (value : UInt8) (accepted : Bool)
    : DecoderResult ByteCursor Span UInt8 =
  match accepted {
    True ↦ Decoded ByteCursor Span UInt8 value (MkByteCursor s (Suc position));
    False ↦
      DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (MkSpan position position))
  }

fn byte_code_decoder_outcome
      (s : Source) (position : Nat) (code : Int) (observed : Option UInt8)
    : DecoderResult ByteCursor Span UInt8 =
  match observed {
    None ↦
      DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (MkSpan position position));
    Some value ↦ byte_code_decoder_accepted s position value (eq_int (uint8_to_int value) code)
  }

theorem byte_code_decoder_preserves
      (code : Int)
    : DecoderPreservesBounded UInt8 (byte_code_decoder code) =
  λs.
    λstart.
      λcur.
        match cur {
          MkByteCursor t position ↦
            λsafe.
              let
                same_bytes : Equal Bytes (source_bytes t) (source_bytes s) =
                  and_fst
                    (Equal Bytes (source_bytes t) (source_bytes s))
                    (And (LessEqNat start position) (LessEqNat position (source_length s)))
                    safe;
                valid_bounds : And
                  (LessEqNat start position)
                  (LessEqNat position (source_length s)) =
                  and_snd
                    (Equal Bytes (source_bytes t) (source_bytes s))
                    (And (LessEqNat start position) (LessEqNat position (source_length s)))
                    safe
              in
                byte_code_decoder_bounded
                  s
                  t
                  start
                  position
                  code
                  same_bytes
                  (and_fst
                    (LessEqNat start position)
                    (LessEqNat position (source_length s))
                    valid_bounds)
                  (and_snd
                    (LessEqNat start position)
                    (LessEqNat position (source_length s))
                    valid_bounds)
        }

theorem byte_code_decoder_outcome_equation
      (s : Source) (position : Nat) (code : Int)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder code (MkByteCursor s position))
        (byte_code_decoder_outcome
          s
          position
          code
          (byte_cursor_peek (MkByteCursor s position))) =
  Refl

theorem byte_code_decoder_bounded
      (s : Source)
      (t : Source)
      (start : Nat)
      (position : Nat)
      (code : Int)
      (same_bytes : Equal Bytes (source_bytes t) (source_bytes s))
      (start_bounded : LessEqNat start position)
      (position_bounded : LessEqNat position (source_length s))
    : DecoderOutcomeBounded UInt8 s start (byte_code_decoder code (MkByteCursor t position)) =
  option_prop_elim
    UInt8
    (λobserved.
      Equal (Option UInt8) (byte_cursor_peek (MkByteCursor t position)) observed
      → DecoderOutcomeBounded
        UInt8
        s
        start
        (byte_code_decoder_outcome t position code observed))
    (byte_cursor_peek (MkByteCursor t position))
    (λnone_peeked. valid_zero_width_span s position position_bounded)
    (λvalue.
      λsome_peeked.
        bool_prop_elim
          (λaccepted.
            DecoderOutcomeBounded
              UInt8
              s
              start
              (byte_code_decoder_accepted t position value accepted))
          (eq_int (uint8_to_int value) code)
          (and_intro
            (Equal Bytes (source_bytes t) (source_bytes s))
            (And (LessEqNat start (Suc position)) (LessEqNat (Suc position) (source_length s)))
            same_bytes
            (byte_cursor_advance_bounded_for_source
              s
              t
              start
              position
              value
              same_bytes
              start_bounded
              some_peeked))
          (valid_zero_width_span s position position_bounded))
    Refl

fn parse_decoder_outcome
      (a : Type) (s : Source) (start : Nat) (outcome : DecoderResult ByteCursor Span a)
    : ParseResult a =
  match outcome {
    Decoded value next ↦
      Parsed a value (MkSpan start (byte_cursor_position next)) (byte_cursor_position next);
    DecoderFailed err ↦ Failed a (decoder_parse_error s err)
  }

theorem parser_from_decoder_outcome_equation
      (a : Type)
      (decoder : Decoder ByteCursor Span a)
      (s : Source)
      (start : Nat)
      (h : LessEqNat start (source_length s))
    : Equal
        (ParseResult a)
        (parser_from_decoder a decoder s start h)
        (parse_decoder_outcome a s start (decoder (MkByteCursor s start))) =
  Refl

theorem decoder_result_prop_elim
      (a : Type)
      (motive : DecoderResult ByteCursor Span a → Prop)
      (outcome : DecoderResult ByteCursor Span a)
      (on_decoded : (value : a)
        → (next : ByteCursor)
        → motive
        (Decoded ByteCursor Span a value next))
      (on_failed : (err : DecoderError Span) → motive (DecoderFailed ByteCursor Span a err))
    : motive outcome =
  match outcome {
    Decoded value next ↦ on_decoded value next;
    DecoderFailed err ↦ on_failed err
  }

theorem parse_decoder_bounded_valid
      (a : Type)
      (s : Source)
      (start : Nat)
      (outcome : DecoderResult ByteCursor Span a)
      (bounded : DecoderOutcomeBounded a s start outcome)
    : ParseResultValid a s start (parse_decoder_outcome a s start outcome) =
  decoder_result_prop_elim
    a
    (λdecoded_outcome.
      DecoderOutcomeBounded a s start decoded_outcome
      → ParseResultValid a s start (parse_decoder_outcome a s start decoded_outcome))
    outcome
    (λvalue.
      λnext.
        λsafe.
          let
            position : Nat = byte_cursor_position next;
            valid_bounds : And
              (LessEqNat start position)
              (LessEqNat position (source_length s)) =
              and_snd
                (Equal Bytes (source_bytes (byte_cursor_source next)) (source_bytes s))
                (And (LessEqNat start position) (LessEqNat position (source_length s)))
                safe;
            span_valid : ValidSpan s (MkSpan start position) =
              and_intro
                (LessEqNat start position)
                (LessEqNat position (source_length s))
                (and_fst
                  (LessEqNat start position)
                  (LessEqNat position (source_length s))
                  valid_bounds)
                (and_snd
                  (LessEqNat start position)
                  (LessEqNat position (source_length s))
                  valid_bounds)
          in
            and_intro
              (ValidSpan s (MkSpan start position))
              (And (Equal Nat start start) (Equal Nat position position))
              span_valid
              (and_intro (Equal Nat start start) (Equal Nat position position) Refl Refl))
    (λerr.
      λsafe.
        and_intro
          (Equal SourceId (source_id s) (source_id s))
          (ValidSpan s (decoder_error_location Span err))
          Refl
          safe)
    bounded

theorem parse_result_valid_source_local
      (a : Type) (s : Source) (start : Nat) (outcome : ParseResult a)
    : ParseResultValid a s start outcome → ParseResultSourceLocal a s outcome =
  match outcome {
    Parsed value consumed next ↦
      λvalid.
        and_fst
          (ValidSpan s consumed)
          (And (Equal Nat (span_start consumed) start) (Equal Nat (span_end consumed) next))
          valid;
    Failed err ↦
      λvalid.
        and_fst
          (Equal SourceId (error_source err) (source_id s))
          (ValidSpan s (error_span err))
          valid
  }

theorem parser_valid_source_local
      (a : Type) (p : Parser a) (valid : ParserValid a p)
    : ParserSourceLocal a p =
  λs. λstart. λh. parse_result_valid_source_local a s start (p s start h) (valid s start h)

theorem parser_from_decoder_valid_if_bounded
      (a : Type)
      (decoder : Decoder ByteCursor Span a)
      (bounded : DecoderPreservesBounded a decoder)
    : ParserValid a (parser_from_decoder a decoder) =
  λs.
    λstart.
      λh.
        parse_decoder_bounded_valid
          a
          s
          start
          (decoder (MkByteCursor s start))
          (bounded
            s
            start
            (MkByteCursor s start)
            (and_intro
              (Equal Bytes (source_bytes s) (source_bytes s))
              (And (LessEqNat start start) (LessEqNat start (source_length s)))
              (bytes_refl (source_bytes s))
              (and_intro
                (LessEqNat start start)
                (LessEqNat start (source_length s))
                ((proof refl for LessEqNat) start)
                h)))

theorem byte_cursor_bounded_locate
      (s : Source) (start : Nat) (cur : ByteCursor)
    : ByteCursorBounded s start cur
      → ValidSpan s (cursor_locate ByteCursor UInt8 Span byte_cursor_ops cur) =
  match cur {
    MkByteCursor t position ↦
      λsafe.
        valid_zero_width_span
          s
          position
          (and_snd
            (LessEqNat start position)
            (LessEqNat position (source_length s))
            (and_snd
              (Equal Bytes (source_bytes t) (source_bytes s))
              (And (LessEqNat start position) (LessEqNat position (source_length s)))
              safe))
  }

theorem byte_cursor_bounded_after_peek
      (s : Source) (start : Nat) (cur : ByteCursor)
    : (value : UInt8)
      → Equal
        (Option UInt8)
        (cursor_peek ByteCursor UInt8 Span byte_cursor_ops cur)
        (Some UInt8 value)
      → ByteCursorBounded s start cur
      → ByteCursorBounded s start (cursor_advance ByteCursor UInt8 Span byte_cursor_ops cur) =
  match cur {
    MkByteCursor t position ↦
      λvalue.
        λpeeked.
          λsafe.
            let
              same_bytes : Equal Bytes (source_bytes t) (source_bytes s) =
                and_fst
                  (Equal Bytes (source_bytes t) (source_bytes s))
                  (And (LessEqNat start position) (LessEqNat position (source_length s)))
                  safe;
              valid_bounds : And
                (LessEqNat start position)
                (LessEqNat position (source_length s)) =
                and_snd
                  (Equal Bytes (source_bytes t) (source_bytes s))
                  (And (LessEqNat start position) (LessEqNat position (source_length s)))
                  safe
            in
              and_intro
                (Equal Bytes (source_bytes t) (source_bytes s))
                (And
                  (LessEqNat start (Suc position))
                  (LessEqNat (Suc position) (source_length s)))
                same_bytes
                (byte_cursor_advance_bounded_for_source
                  s
                  t
                  start
                  position
                  value
                  same_bytes
                  (and_fst
                    (LessEqNat start position)
                    (LessEqNat position (source_length s))
                    valid_bounds)
                  peeked)
  }

theorem decoder_bounded_as_public
      (a : Type)
      (decoder : Decoder ByteCursor Span a)
      (s : Source)
      (start : Nat)
      (bounded : DecoderPreservesBounded a decoder)
    : DecoderPreserves ByteCursor Span a (ByteCursorBounded s start) (ValidSpan s) decoder =
  λcur. λgood. bounded s start cur good

theorem decoder_public_as_bounded
      (a : Type)
      (decoder : Decoder ByteCursor Span a)
      (preserves : (s : Source)
        → (start : Nat)
        → DecoderPreserves
        ByteCursor
        Span
        a
        (ByteCursorBounded s start)
        (ValidSpan s)
        decoder)
    : DecoderPreservesBounded a decoder =
  λs. λstart. λcur. λgood. preserves s start cur good

theorem byte_code_decoder_public_preserves
      (s : Source) (start : Nat) (code : Int)
    : DecoderPreserves ByteCursor Span UInt8
        (ByteCursorBounded s start)
        (ValidSpan s)
        (byte_code_decoder code) =
  decoder_satisfy_preserves
    ByteCursor
    UInt8
    Span
    byte_cursor_ops
    (λbyte. eq_int (uint8_to_int byte) code)
    (ByteCursorBounded s start)
    (ValidSpan s)
    (byte_cursor_bounded_locate s start)
    (byte_cursor_bounded_after_peek s start)

theorem byte_code_decoder_public_bounded
      (code : Int)
    : DecoderPreservesBounded UInt8 (byte_code_decoder code) =
  decoder_public_as_bounded
    UInt8
    (byte_code_decoder code)
    (λs. λstart. byte_code_decoder_public_preserves s start code)

theorem decoder_seq_public_bounded
      (a : Type)
      (b : Type)
      (first : Decoder ByteCursor Span a)
      (second : Decoder ByteCursor Span b)
      (first_safe : DecoderPreservesBounded a first)
      (second_safe : DecoderPreservesBounded b second)
    : DecoderPreservesBounded b (decoder_seq ByteCursor Span a b first second) =
  decoder_public_as_bounded
    b
    (decoder_seq ByteCursor Span a b first second)
    (λs.
      λstart.
        decoder_seq_preserves
          ByteCursor
          Span
          a
          b
          (ByteCursorBounded s start)
          (ValidSpan s)
          first
          second
          (decoder_bounded_as_public a first s start first_safe)
          (decoder_bounded_as_public b second s start second_safe))

theorem prepend_byte_token_bounded
      (code : Int)
      (tail : Decoder ByteCursor Span UInt8)
      (tail_safe : DecoderPreservesBounded UInt8 tail)
    : DecoderPreservesBounded UInt8
        (decoder_seq ByteCursor Span UInt8 UInt8 (byte_code_decoder code) tail) =
  decoder_seq_public_bounded
    UInt8
    UInt8
    (byte_code_decoder code)
    tail
    (byte_code_decoder_public_bounded code)
    tail_safe

theorem four_byte_token_bounded
      (first : Int) (second : Int) (third : Int) (fourth : Int)
    : DecoderPreservesBounded UInt8
        (decoder_seq
          ByteCursor
          Span
          UInt8
          UInt8
          (byte_code_decoder first)
          (decoder_seq
            ByteCursor
            Span
            UInt8
            UInt8
            (byte_code_decoder second)
            (decoder_seq
              ByteCursor
              Span
              UInt8
              UInt8
              (byte_code_decoder third)
              (byte_code_decoder fourth)))) =
  let
    last_two : Decoder ByteCursor Span UInt8 =
      decoder_seq
        ByteCursor
        Span
        UInt8
        UInt8
        (byte_code_decoder third)
        (byte_code_decoder fourth);
    last_two_safe : DecoderPreservesBounded UInt8 last_two =
      prepend_byte_token_bounded
        third
        (byte_code_decoder fourth)
        (byte_code_decoder_public_bounded fourth);
    last_three : Decoder ByteCursor Span UInt8 =
      decoder_seq ByteCursor Span UInt8 UInt8 (byte_code_decoder second) last_two;
    last_three_safe : DecoderPreservesBounded UInt8 last_three =
      prepend_byte_token_bounded second last_two last_two_safe
  in
    prepend_byte_token_bounded first last_three last_three_safe

theorem five_byte_token_bounded
      (first : Int) (second : Int) (third : Int) (fourth : Int) (fifth : Int)
    : DecoderPreservesBounded UInt8
        (decoder_seq
          ByteCursor
          Span
          UInt8
          UInt8
          (byte_code_decoder first)
          (decoder_seq
            ByteCursor
            Span
            UInt8
            UInt8
            (byte_code_decoder second)
            (decoder_seq
              ByteCursor
              Span
              UInt8
              UInt8
              (byte_code_decoder third)
              (decoder_seq
                ByteCursor
                Span
                UInt8
                UInt8
                (byte_code_decoder fourth)
                (byte_code_decoder fifth))))) =
  prepend_byte_token_bounded
    first
    (decoder_seq
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder second)
      (decoder_seq
        ByteCursor
        Span
        UInt8
        UInt8
        (byte_code_decoder third)
        (decoder_seq
          ByteCursor
          Span
          UInt8
          UInt8
          (byte_code_decoder fourth)
          (byte_code_decoder fifth))))
    (four_byte_token_bounded second third fourth fifth)

theorem true_token_bounded : DecoderPreservesBounded UInt8 true_token_decoder =
  four_byte_token_bounded (116 : Int) (114 : Int) (117 : Int) (101 : Int)

theorem false_token_bounded : DecoderPreservesBounded UInt8 false_token_decoder =
  five_byte_token_bounded (102 : Int) (97 : Int) (108 : Int) (115 : Int) (101 : Int)

theorem not_open_token_bounded : DecoderPreservesBounded UInt8 not_open_token_decoder =
  five_byte_token_bounded (40 : Int) (110 : Int) (111 : Int) (116 : Int) (32 : Int)

theorem and_open_token_bounded : DecoderPreservesBounded UInt8 and_open_token_decoder =
  five_byte_token_bounded (40 : Int) (97 : Int) (110 : Int) (100 : Int) (32 : Int)

theorem spaces_decoder_bounded : DecoderPreservesBounded (List UInt8) spaces_decoder =
  decoder_public_as_bounded
    (List UInt8)
    spaces_decoder
    (λs.
      λstart.
        decoder_many_preserves
          ByteCursor
          UInt8
          Span
          UInt8
          byte_cursor_ops
          (byte_code_decoder (32 : Int))
          (ByteCursorBounded s start)
          (ValidSpan s)
          (byte_cursor_bounded_locate s start)
          (byte_code_decoder_public_preserves s start (32 : Int)))

fn decoder_then_result
      (a : Type)
      (b : Type)
      (first_outcome : DecoderResult ByteCursor Span a)
      (continue : a → ByteCursor → DecoderResult ByteCursor Span b)
    : DecoderResult ByteCursor Span b =
  match first_outcome {
    DecoderFailed err ↦ DecoderFailed ByteCursor Span b err;
    Decoded value next ↦ continue value next
  }

theorem decoder_then_result_bounded
      (a : Type)
      (b : Type)
      (s : Source)
      (start : Nat)
      (first_outcome : DecoderResult ByteCursor Span a)
      (continue : a → ByteCursor → DecoderResult ByteCursor Span b)
      (first_safe : DecoderOutcomeBounded a s start first_outcome)
      (continue_safe : (value : a)
        → (next : ByteCursor)
        → ByteCursorBounded
        s
        start
        next
        → DecoderOutcomeBounded
        b
        s
        start
        (continue value next))
    : DecoderOutcomeBounded b s start (decoder_then_result a b first_outcome continue) =
  decoder_result_prop_elim
    a
    (λoutcome.
      DecoderOutcomeBounded a s start outcome
      → DecoderOutcomeBounded b s start (decoder_then_result a b outcome continue))
    first_outcome
    (λvalue. λnext. λsafe. continue_safe value next safe)
    (λerr. λsafe. safe)
    first_safe

fn decoder_then_decoded
      (a : Type)
      (b : Type)
      (outcome : DecoderResult ByteCursor Span a)
      (build : a → ByteCursor → b)
    : DecoderResult ByteCursor Span b =
  decoder_then_result
    a
    b
    outcome
    (λvalue. λnext. Decoded ByteCursor Span b (build value next) next)

theorem decoder_then_decoded_bounded
      (a : Type)
      (b : Type)
      (s : Source)
      (start : Nat)
      (outcome : DecoderResult ByteCursor Span a)
      (build : a → ByteCursor → b)
      (safe : DecoderOutcomeBounded a s start outcome)
    : DecoderOutcomeBounded b s start (decoder_then_decoded a b outcome build) =
  decoder_then_result_bounded
    a
    b
    s
    start
    outcome
    (λvalue. λnext. Decoded ByteCursor Span b (build value next) next)
    safe
    (λvalue. λnext. λnext_safe. next_safe)

theorem bool_true_decoder_bounded
    : DecoderPreservesBounded (Syntax BoolExpr) bool_true_decoder =
  λs.
    λstart.
      λcur.
        λsafe.
          decoder_then_result_bounded
            UInt8
            (Syntax BoolExpr)
            s
            start
            (true_token_decoder cur)
            (λignored.
              λnext.
                Decoded
                  ByteCursor
                  Span
                  (Syntax BoolExpr)
                  (syntax_leaf
                    (byte_cursor_source cur)
                    (byte_cursor_position cur)
                    (byte_cursor_position next)
                    BTrue)
                  next)
            (true_token_bounded s start cur safe)
            (λignored. λnext. λnext_safe. next_safe)

theorem bool_false_decoder_bounded
    : DecoderPreservesBounded (Syntax BoolExpr) bool_false_decoder =
  λs.
    λstart.
      λcur.
        λsafe.
          decoder_then_result_bounded
            UInt8
            (Syntax BoolExpr)
            s
            start
            (false_token_decoder cur)
            (λignored.
              λnext.
                Decoded
                  ByteCursor
                  Span
                  (Syntax BoolExpr)
                  (syntax_leaf
                    (byte_cursor_source cur)
                    (byte_cursor_position cur)
                    (byte_cursor_position next)
                    BFalse)
                  next)
            (false_token_bounded s start cur safe)
            (λignored. λnext. λnext_safe. next_safe)

fn not_close_after
      (cur : ByteCursor) (child : Syntax BoolExpr) (close_start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_decoded
    UInt8
    (Syntax BoolExpr)
    (byte_code_decoder (41 : Int) close_start)
    (λclose.
      λafter_close.
        syntax_node_unary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position after_close)
          (BNot (erase_spans child))
          child)

theorem not_close_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (child : Syntax BoolExpr)
      (close_start : ByteCursor)
      (safe : ByteCursorBounded s start close_start)
    : DecoderOutcomeBounded (Syntax BoolExpr) s start (not_close_after cur child close_start) =
  decoder_then_decoded_bounded
    UInt8
    (Syntax BoolExpr)
    s
    start
    (byte_code_decoder (41 : Int) close_start)
    (λclose.
      λafter_close.
        syntax_node_unary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position after_close)
          (BNot (erase_spans child))
          child)
    (byte_code_decoder_public_bounded (41 : Int) s start close_start safe)

fn not_trailing_after
      (cur : ByteCursor) (child : Syntax BoolExpr) (child_end : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (List UInt8)
    (Syntax BoolExpr)
    (spaces_decoder child_end)
    (λtrailing. λclose_start. not_close_after cur child close_start)

theorem not_trailing_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (child : Syntax BoolExpr)
      (child_end : ByteCursor)
      (safe : ByteCursorBounded s start child_end)
    : DecoderOutcomeBounded (Syntax BoolExpr) s start (not_trailing_after cur child child_end) =
  decoder_then_result_bounded
    (List UInt8)
    (Syntax BoolExpr)
    s
    start
    (spaces_decoder child_end)
    (λtrailing. λclose_start. not_close_after cur child close_start)
    (spaces_decoder_bounded s start child_end safe)
    (λtrailing.
      λclose_start.
        λclose_safe. not_close_after_bounded s start cur child close_start close_safe)

fn not_child_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (child_start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    (recur child_start)
    (λchild. λchild_end. not_trailing_after cur child child_end)

theorem not_child_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (child_start : ByteCursor)
      (safe : ByteCursorBounded s start child_start)
    : DecoderOutcomeBounded (Syntax BoolExpr) s start (not_child_after cur recur child_start) =
  decoder_then_result_bounded
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    s
    start
    (recur child_start)
    (λchild. λchild_end. not_trailing_after cur child child_end)
    (recur_safe child_start safe)
    (λchild.
      λchild_end.
        λchild_end_safe. not_trailing_after_bounded s start cur child child_end child_end_safe)

fn not_spaces_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (after_open : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (List UInt8)
    (Syntax BoolExpr)
    (spaces_decoder after_open)
    (λspaces. λchild_start. not_child_after cur recur child_start)

theorem not_spaces_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (after_open : ByteCursor)
      (safe : ByteCursorBounded s start after_open)
    : DecoderOutcomeBounded (Syntax BoolExpr) s start (not_spaces_after cur recur after_open) =
  decoder_then_result_bounded
    (List UInt8)
    (Syntax BoolExpr)
    s
    start
    (spaces_decoder after_open)
    (λspaces. λchild_start. not_child_after cur recur child_start)
    (spaces_decoder_bounded s start after_open safe)
    (λspaces.
      λchild_start.
        λchild_start_safe.
          not_child_after_bounded s start cur recur recur_safe child_start child_start_safe)

theorem bool_not_decoder_bounded
      (s : Source)
      (start : Nat)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
    : DecoderPreserves ByteCursor Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        (bool_not_decoder recur) =
  λcur.
    λsafe.
      decoder_then_result_bounded
        UInt8
        (Syntax BoolExpr)
        s
        start
        (not_open_token_decoder cur)
        (λignored. λafter_open. not_spaces_after cur recur after_open)
        (not_open_token_bounded s start cur safe)
        (λignored.
          λafter_open.
            λafter_open_safe.
              not_spaces_after_bounded s start cur recur recur_safe after_open after_open_safe)

fn and_close_after
      (cur : ByteCursor)
      (left : Syntax BoolExpr)
      (right : Syntax BoolExpr)
      (close_start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_decoded
    UInt8
    (Syntax BoolExpr)
    (byte_code_decoder (41 : Int) close_start)
    (λclose.
      λafter_close.
        syntax_node_binary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position after_close)
          (BAnd (erase_spans left) (erase_spans right))
          left
          right)

theorem and_close_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (left : Syntax BoolExpr)
      (right : Syntax BoolExpr)
      (close_start : ByteCursor)
      (safe : ByteCursorBounded s start close_start)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (and_close_after cur left right close_start) =
  decoder_then_decoded_bounded
    UInt8
    (Syntax BoolExpr)
    s
    start
    (byte_code_decoder (41 : Int) close_start)
    (λclose.
      λafter_close.
        syntax_node_binary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position after_close)
          (BAnd (erase_spans left) (erase_spans right))
          left
          right)
    (byte_code_decoder_public_bounded (41 : Int) s start close_start safe)

fn and_trailing_after
      (cur : ByteCursor)
      (left : Syntax BoolExpr)
      (right : Syntax BoolExpr)
      (right_end : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (List UInt8)
    (Syntax BoolExpr)
    (spaces_decoder right_end)
    (λtrailing. λclose_start. and_close_after cur left right close_start)

theorem and_trailing_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (left : Syntax BoolExpr)
      (right : Syntax BoolExpr)
      (right_end : ByteCursor)
      (safe : ByteCursorBounded s start right_end)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (and_trailing_after cur left right right_end) =
  decoder_then_result_bounded
    (List UInt8)
    (Syntax BoolExpr)
    s
    start
    (spaces_decoder right_end)
    (λtrailing. λclose_start. and_close_after cur left right close_start)
    (spaces_decoder_bounded s start right_end safe)
    (λtrailing.
      λclose_start.
        λclose_safe. and_close_after_bounded s start cur left right close_start close_safe)

fn and_right_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (left : Syntax BoolExpr)
      (right_start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    (recur right_start)
    (λright. λright_end. and_trailing_after cur left right right_end)

theorem and_right_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (left : Syntax BoolExpr)
      (right_start : ByteCursor)
      (safe : ByteCursorBounded s start right_start)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (and_right_after cur recur left right_start) =
  decoder_then_result_bounded
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    s
    start
    (recur right_start)
    (λright. λright_end. and_trailing_after cur left right right_end)
    (recur_safe right_start safe)
    (λright.
      λright_end.
        λright_end_safe.
          and_trailing_after_bounded s start cur left right right_end right_end_safe)

fn and_middle_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (left : Syntax BoolExpr)
      (after_separator : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (List UInt8)
    (Syntax BoolExpr)
    (spaces_decoder after_separator)
    (λmiddle. λright_start. and_right_after cur recur left right_start)

theorem and_middle_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (left : Syntax BoolExpr)
      (after_separator : ByteCursor)
      (safe : ByteCursorBounded s start after_separator)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (and_middle_after cur recur left after_separator) =
  decoder_then_result_bounded
    (List UInt8)
    (Syntax BoolExpr)
    s
    start
    (spaces_decoder after_separator)
    (λmiddle. λright_start. and_right_after cur recur left right_start)
    (spaces_decoder_bounded s start after_separator safe)
    (λmiddle.
      λright_start.
        λright_start_safe.
          and_right_after_bounded
            s
            start
            cur
            recur
            recur_safe
            left
            right_start
            right_start_safe)

fn and_separator_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (left : Syntax BoolExpr)
      (left_end : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    UInt8
    (Syntax BoolExpr)
    (byte_code_decoder (32 : Int) left_end)
    (λseparator. λafter_separator. and_middle_after cur recur left after_separator)

theorem and_separator_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (left : Syntax BoolExpr)
      (left_end : ByteCursor)
      (safe : ByteCursorBounded s start left_end)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (and_separator_after cur recur left left_end) =
  decoder_then_result_bounded
    UInt8
    (Syntax BoolExpr)
    s
    start
    (byte_code_decoder (32 : Int) left_end)
    (λseparator. λafter_separator. and_middle_after cur recur left after_separator)
    (byte_code_decoder_public_bounded (32 : Int) s start left_end safe)
    (λseparator.
      λafter_separator.
        λafter_separator_safe.
          and_middle_after_bounded
            s
            start
            cur
            recur
            recur_safe
            left
            after_separator
            after_separator_safe)

fn and_left_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (left_start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    (recur left_start)
    (λleft. λleft_end. and_separator_after cur recur left left_end)

theorem and_left_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (left_start : ByteCursor)
      (safe : ByteCursorBounded s start left_start)
    : DecoderOutcomeBounded (Syntax BoolExpr) s start (and_left_after cur recur left_start) =
  decoder_then_result_bounded
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    s
    start
    (recur left_start)
    (λleft. λleft_end. and_separator_after cur recur left left_end)
    (recur_safe left_start safe)
    (λleft.
      λleft_end.
        λleft_end_safe.
          and_separator_after_bounded s start cur recur recur_safe left left_end left_end_safe)

fn and_leading_after
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (after_open : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (List UInt8)
    (Syntax BoolExpr)
    (spaces_decoder after_open)
    (λleading. λleft_start. and_left_after cur recur left_start)

theorem and_leading_after_bounded
      (s : Source)
      (start : Nat)
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
      (after_open : ByteCursor)
      (safe : ByteCursorBounded s start after_open)
    : DecoderOutcomeBounded (Syntax BoolExpr) s start (and_leading_after cur recur after_open) =
  decoder_then_result_bounded
    (List UInt8)
    (Syntax BoolExpr)
    s
    start
    (spaces_decoder after_open)
    (λleading. λleft_start. and_left_after cur recur left_start)
    (spaces_decoder_bounded s start after_open safe)
    (λleading.
      λleft_start.
        λleft_start_safe.
          and_left_after_bounded s start cur recur recur_safe left_start left_start_safe)

theorem bool_and_decoder_bounded
      (s : Source)
      (start : Nat)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
    : DecoderPreserves ByteCursor Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        (bool_and_decoder recur) =
  λcur.
    λsafe.
      decoder_then_result_bounded
        UInt8
        (Syntax BoolExpr)
        s
        start
        (and_open_token_decoder cur)
        (λignored. λafter_open. and_leading_after cur recur after_open)
        (and_open_token_bounded s start cur safe)
        (λignored.
          λafter_open.
            λafter_open_safe.
              and_leading_after_bounded s start cur recur recur_safe after_open after_open_safe)

theorem bool_decoder_layer_bounded
      (s : Source)
      (start : Nat)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (recur_safe : DecoderPreserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        recur)
    : DecoderPreserves ByteCursor Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        (bool_decoder_layer recur) =
  let
    recursive_alternatives : Decoder ByteCursor Span (Syntax BoolExpr) =
      decoder_alt
        ByteCursor
        Span
        (Syntax BoolExpr)
        (bool_not_decoder recur)
        (bool_and_decoder recur);
    recursive_alternatives_safe : DecoderPreserves ByteCursor Span
      (Syntax BoolExpr)
      (ByteCursorBounded s start)
      (ValidSpan s)
      recursive_alternatives =
      decoder_alt_preserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        (bool_not_decoder recur)
        (bool_and_decoder recur)
        (bool_not_decoder_bounded s start recur recur_safe)
        (bool_and_decoder_bounded s start recur recur_safe);
    other_alternatives : Decoder ByteCursor Span (Syntax BoolExpr) =
      decoder_alt ByteCursor Span (Syntax BoolExpr) bool_false_decoder recursive_alternatives;
    other_alternatives_safe : DecoderPreserves ByteCursor Span
      (Syntax BoolExpr)
      (ByteCursorBounded s start)
      (ValidSpan s)
      other_alternatives =
      decoder_alt_preserves
        ByteCursor
        Span
        (Syntax BoolExpr)
        (ByteCursorBounded s start)
        (ValidSpan s)
        bool_false_decoder
        recursive_alternatives
        (decoder_bounded_as_public
          (Syntax BoolExpr)
          bool_false_decoder
          s
          start
          bool_false_decoder_bounded)
        recursive_alternatives_safe
  in
    decoder_alt_preserves
      ByteCursor
      Span
      (Syntax BoolExpr)
      (ByteCursorBounded s start)
      (ValidSpan s)
      bool_true_decoder
      other_alternatives
      (decoder_bounded_as_public
        (Syntax BoolExpr)
        bool_true_decoder
        s
        start
        bool_true_decoder_bounded)
      other_alternatives_safe

theorem bool_expression_decoder_bounded
    : DecoderPreservesBounded (Syntax BoolExpr) bool_expression_decoder =
  decoder_public_as_bounded
    (Syntax BoolExpr)
    bool_expression_decoder
    (λs.
      λstart.
        decoder_recursive_preserves
          ByteCursor
          UInt8
          Span
          (Syntax BoolExpr)
          byte_cursor_ops
          bool_decoder_layer
          (ByteCursorBounded s start)
          (ValidSpan s)
          (byte_cursor_bounded_locate s start)
          (λrecur. λrecur_safe. bool_decoder_layer_bounded s start recur recur_safe))

fn complete_bool_finish_result
      (syntax : Syntax BoolExpr) (end : ByteCursor) (remaining : Nat)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  match remaining {
    Zero ↦ Decoded ByteCursor Span (Syntax BoolExpr) syntax end;
    Suc rest ↦
      DecoderFailed
        ByteCursor
        Span
        (Syntax BoolExpr)
        (DecoderRejected Span (byte_cursor_locate end))
  }

fn complete_bool_finish
      (syntax : Syntax BoolExpr) (end : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  complete_bool_finish_result syntax end (byte_cursor_remaining end)

theorem nat_prop_elim
      (motive : Nat → Prop)
      (n : Nat)
      (on_zero : motive Zero)
      (on_successor : (previous : Nat) → motive (Suc previous))
    : motive n =
  match n {
    Zero ↦ on_zero;
    Suc previous ↦ on_successor previous
  }

theorem complete_bool_finish_bounded
      (s : Source) (start : Nat) (syntax : Syntax BoolExpr) (end : ByteCursor)
    : ByteCursorBounded s start end
      → DecoderOutcomeBounded (Syntax BoolExpr) s start (complete_bool_finish syntax end) =
  nat_prop_elim
    (λremaining.
      ByteCursorBounded s start end
      → DecoderOutcomeBounded
        (Syntax BoolExpr)
        s
        start
        (complete_bool_finish_result syntax end remaining))
    (byte_cursor_remaining end)
    (λsafe. safe)
    (λrest. λsafe. byte_cursor_bounded_locate s start end safe)

fn complete_bool_trailing_after
      (syntax : Syntax BoolExpr) (next : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (List UInt8)
    (Syntax BoolExpr)
    (spaces_decoder next)
    (λtrailing. λend. complete_bool_finish syntax end)

theorem complete_bool_trailing_after_bounded
      (s : Source)
      (start : Nat)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (safe : ByteCursorBounded s start next)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (complete_bool_trailing_after syntax next) =
  decoder_then_result_bounded
    (List UInt8)
    (Syntax BoolExpr)
    s
    start
    (spaces_decoder next)
    (λtrailing. λend. complete_bool_finish syntax end)
    (spaces_decoder_bounded s start next safe)
    (λtrailing. λend. λend_safe. complete_bool_finish_bounded s start syntax end end_safe)

fn complete_bool_expression_after
      (grammar_start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  decoder_then_result
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    (bool_expression_decoder grammar_start)
    complete_bool_trailing_after

theorem complete_bool_expression_after_bounded
      (s : Source)
      (start : Nat)
      (grammar_start : ByteCursor)
      (safe : ByteCursorBounded s start grammar_start)
    : DecoderOutcomeBounded
        (Syntax BoolExpr)
        s start
        (complete_bool_expression_after grammar_start) =
  decoder_then_result_bounded
    (Syntax BoolExpr)
    (Syntax BoolExpr)
    s
    start
    (bool_expression_decoder grammar_start)
    complete_bool_trailing_after
    (bool_expression_decoder_bounded s start grammar_start safe)
    (λsyntax.
      λnext. λnext_safe. complete_bool_trailing_after_bounded s start syntax next next_safe)

theorem complete_bool_decoder_bounded
    : DecoderPreservesBounded (Syntax BoolExpr) complete_bool_decoder =
  λs.
    λstart.
      λcur.
        λsafe.
          decoder_then_result_bounded
            (List UInt8)
            (Syntax BoolExpr)
            s
            start
            (spaces_decoder cur)
            (λleading. λgrammar_start. complete_bool_expression_after grammar_start)
            (spaces_decoder_bounded s start cur safe)
            (λleading.
              λgrammar_start.
                λgrammar_safe.
                  complete_bool_expression_after_bounded s start grammar_start grammar_safe)

pub theorem parse_bool_expr_laws : ParserLaws (Syntax BoolExpr) parse_bool_expr =
  parse_bool_expr_laws_if_decoder_bounded complete_bool_decoder_bounded

theorem parse_bool_expr_laws_if_decoder_bounded
      (bounded : DecoderPreservesBounded (Syntax BoolExpr) complete_bool_decoder)
    : ParserLaws (Syntax BoolExpr) parse_bool_expr =
  let valid : ParserValid (Syntax BoolExpr) parse_bool_expr =
    parser_from_decoder_valid_if_bounded (Syntax BoolExpr) complete_bool_decoder bounded
  in
    and_intro
      (ParserValid (Syntax BoolExpr) parse_bool_expr)
      (And
        (ParserTotal (Syntax BoolExpr) parse_bool_expr)
        (ParserSourceLocal (Syntax BoolExpr) parse_bool_expr))
      valid
      (and_intro
        (ParserTotal (Syntax BoolExpr) parse_bool_expr)
        (ParserSourceLocal (Syntax BoolExpr) parse_bool_expr)
        parse_bool_expr_total
        (parser_valid_source_local (Syntax BoolExpr) parse_bool_expr valid))

pub theorem format_bool_expr_on_parse_success
      (s : Source)
      (syntax : Syntax BoolExpr)
      (consumed : Span)
      (next : Nat)
      (h : LessEqNat Zero (source_length s))
      (parsed : Equal
        (ParseResult (Syntax BoolExpr))
        (parse_bool_expr s Zero h)
        (Parsed (Syntax BoolExpr) syntax consumed next))
    : Equal
        (Result ParseError Bytes)
        (format_bool_expr s)
        (Ok ParseError Bytes (print_bool_expr (erase_spans syntax))) =
  J
    (λoutcome _.
      Equal (Result ParseError Bytes) (format_bool_expr s) (format_bool_parse_outcome outcome))
    Refl
    parsed

pub theorem format_bool_expr_on_parse_failure
      (s : Source)
      (err : ParseError)
      (h : LessEqNat Zero (source_length s))
      (failed : Equal
        (ParseResult (Syntax BoolExpr))
        (parse_bool_expr s Zero h)
        (Failed (Syntax BoolExpr) err))
    : Equal (Result ParseError Bytes) (format_bool_expr s) (Err ParseError Bytes err) =
  J
    (λoutcome _.
      Equal (Result ParseError Bytes) (format_bool_expr s) (format_bool_parse_outcome outcome))
    Refl
    failed
```

## 5. Design notes

**Why `Bytes`, never `String`, as the offset basis.** Every position and
length in this package is a `Bytes`/`Int` quantity; `IsUtf8` is carried as
evidence the bytes happen to decode losslessly, never as a precondition
positions are computed against. A codepoint-indexed offset would need a
decode step (and a failure mode) just to compute `span_end - span_start`;
a byte-indexed offset never does.

**`ParseResultTotal`/`ParserTotal` are honestly weak.** Both match arms of
`ParseResultTotal` reduce to the same `Top` — the only
content this witnesses is that the `Parsed`/`Failed` case-split is
exhaustive, not any deeper property of a particular parser. A reader should
not over-read "Total" here as more than "this function always returns one
of its two constructors," which the type checker already guarantees; see
`§6`.

**Why repetition is progress checked.** `Capability.Parsing.Decoder` exports repetition,
but a successful step must strictly decrease the cursor's remaining count.
`decoder_many` reports the named `DecoderZeroProgress` failure when that
check fails and derives its private fuel from `remaining`, so it neither loops
nor truncates at an arbitrary caller budget. CAT-5's whitespace decoder is a
direct client of that shared mechanism.

**Why the Boolean grammar has no precedence table.** `true`, `false`,
`(not e)`, and `(and e1 e2)` are fully parenthesized on purpose — `true and
false` rejects, deliberately, keeping this worked example small. A real
expression grammar with precedence climbing is future package work, not
attempted here.

## 6. References

None — this entry's design is Ken-native, not consulted from an external
reference implementation.

## 7. Trust  derivation

1. **Public API.** `Source`, `IsUtf8`, `source_id`, `source_bytes`,
   `source_bytes::utf8`, `source_length`, `Span`, `MkSpan`, `span_start`,
   `span_end`, `span_to_byte_range`, `span_origin`, `ByteCursor`,
   `byte_cursor_ops`, `LessEqNat`, `LessEqNat::refl`,
   `LessEqNat::zero_left`, `ValidSpan`, `valid_zero_width_span`, `Located`,
   `MkLocated`, `located_source`, `located_span`, `located_value`,
   `ValidLocated`, `ParseError`, `MkParseError`, `error_source`, `error_span`,
   `ParseResult`, `Parsed`, `Failed`, `Parser`, `ParsedValid`, `FailedValid`,
   `ParseResultValid`, `ParserValid`, `ParserTotal`, `ParserSourceLocal`,
   `ParserLaws`, `parser_from_decoder`, `parser_pure`, `parser_fail`,
   `BoolExpr`, `BTrue`, `BFalse`, `BNot`, `BAnd`, `Syntax`, `MkSyntax`,
   `syntax_root`, `syntax_children`, `erase_spans`, `ValidLocatedList`,
   `ValidSyntax`, `parse_bool_expr`, `parse_bool_expr_total`,
   `parse_bool_expr_laws`, `print_bool_expr`, `format_bool_expr`,
   `format_bool_expr_on_parse_success`, and
   `format_bool_expr_on_parse_failure`.
2. **Source map.**

   | Task | Section |
   |---|---|
   | See the source/span/parser vocabulary | [Definition](#2-definition) |
   | Build a `Source`, drive the grammar | [Using it](#3-using-it) |
   | The zero-width-span proof, the total `Parser` contract, the worked grammar | [Laws  proofs](#4-laws--proofs) |
   | Why `Bytes` not `String`, why no unguarded repetition | [Design notes](#5-design-notes) |

3. **Derivation path.** The package surface is ordinary Ken data, a
   class-backed record, transparent functions, and proof-returning
   definitions over `Nat`, `Bool`, `Bytes`, `Equal`, `And`, `List`, and
   parser-result data. It adds no kernel primitive, no source-loader
   behavior, and no language-semantics change.
4. **`trusted_base()` delta.** **Zero.** Every proof in this package —
   `LessEqNat::refl`, `LessEqNat::zero_left`, `valid_zero_width_span`,
   `parse_bool_expr_total`, `parse_bool_expr_laws`,
   `format_bool_expr_on_parse_success`, and
   `format_bool_expr_on_parse_failure` — is real and kernel-checked; no law
   or predicate is postulated.
5. **Proof families.** `LessEqNat::refl` — induction on `n`.
   `LessEqNat::zero_left` — definitional (first match arm). `valid_zero_width_span`
   — direct composition of the two via `and_intro`, no case-split of its
   own. `parse_bool_expr_total` — exhaustive parse-result split.
   `parse_bool_expr_laws` — Decoder preservation instantiated with the
   source-bounded cursor, checked `nth`/`leq_nat` bounds, and a private
   outcome-to-parser-validity bridge; `decoder_recursive_preserves`
   handles fuel internally. `format_bool_expr_on_parse_success` and
   `format_bool_expr_on_parse_failure` — equality transport across each
   parser-result alternative. These equations do not assert that printing
   then parsing returns the original expression.
6. **Consumers.** Source-aware parser implementations can use this package's
   source, span, result, and validity vocabulary.
7. **Validation evidence.** The catalog checks the
   `Source`/`Span`/`Located`/`ParseResult`/`Parser` surface, its zero
   `trusted_base()` delta, the Boolean grammar's constructors and byte-token
   matching, and the absence of an exported unguarded repetition combinator.
