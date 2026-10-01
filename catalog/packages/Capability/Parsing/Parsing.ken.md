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
  (CursorOps,
    MkCursorOps,
    cursor_advance,
    cursor_locate,
    cursor_nat_lt,
    cursor_peek,
    cursor_remaining)

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
    decoder_alt_rejection_uses_second,
    decoder_error_location,
    decoder_fail,
    decoder_many,
    decoder_many_preserves,
    decoder_many_rejected_succeeds,
    decoder_pure,
    decoder_recursive,
    decoder_recursive_preserves,
    decoder_recursive_succeeds,
    decoder_satisfy,
    decoder_satisfy_preserves,
    decoder_seq,
    decoder_seq_preserves)

import Core.Classes.LawfulClasses (bytes_to_list_injective, leq_nat)

import Core.Logic.Transport (cong, sym, trans)

import Data.Binary.BytesPrimitiveContracts
  (AllAscii,
    AllAsciiCodes,
    AsciiBytes,
    NoCodes,
    SomeCodes,
    MkAsciiCode,
    Ascii32,
    Ascii40,
    Ascii41,
    Ascii97,
    Ascii100,
    Ascii101,
    Ascii102,
    Ascii108,
    Ascii110,
    Ascii111,
    Ascii114,
    Ascii115,
    Ascii116,
    Ascii117,
    ascii_bytes_utf8,
    bytes_concat_list_view,
    bytes_encode_ascii_octets)

import Data.Collections.Derived (bytes_nat_length, length, list_append, map, nth)

import Data.Numeric.Nat.Order (leq_nat_successor_bound, sub)

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
a demo. `parse_bool_expr_print_round_trip` proves the parsed expression
erases to the printed input rather than failing;
`format_bool_expr_print_round_trip` proves that formatting its printed
source returns the original bytes.

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
always fails at `start` with a zero-width error span. Every
`parser_from_decoder` parser whose decoder preserves source bounds has
`ParserLaws`: its validity follows from the bounded decoder outcome, totality
from the two result constructors, and source locality from validity.
`parser_pure` preserves the input cursor; `parser_fail` reports its zero-width
error at that cursor. Both satisfy the required bound, so their public laws are
instances of the generic theorem. The Boolean parser's `parse_bool_expr_total`
inhabits `ParserTotal` on its own; `parse_bool_expr_laws` applies the same
generic theorem to its checked decoder bound.

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
```

### 4.3 Bounded parsers and a worked Boolean grammar

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
end-of-input check. `parse_bool_expr_laws` applies `parser_from_decoder_laws`
to the Boolean decoder's checked bound, retaining the unweakened
`ParserValid`, `ParserTotal`, and `ParserSourceLocal` contract.
The printer's six token strings have one private identity each, shared by
printing and their checked ASCII witnesses. String literals are opaque values:
two separately written literals with the same spelling need a proof of their
equality. The byte and UTF-8 bridges below establish that printed expressions
form valid source bytes. The round-trip proof follows the printed tokens,
children, and cursor suffixes through the recursive decoder: parsing succeeds
and erases to the original expression, and formatting restores its bytes.

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
  decoder_many ByteCursor UInt8 Span UInt8 byte_cursor_ops (byte_code_decoder separator_code)

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

const true_token_text = "true"

const false_token_text = "false"

const not_open_text = "(not "

const and_open_text = "(and "

const separator_text = " "

const close_text = ")"

pub fn print_bool_expr (e : BoolExpr) : Bytes =
  match e {
    BTrue ↦ bytes_encode true_token_text;
    BFalse ↦ bytes_encode false_token_text;
    BNot child ↦
      bytes_concat
        (bytes_concat (bytes_encode not_open_text) (print_bool_expr child))
        (bytes_encode close_text);
    BAnd left right ↦
      bytes_concat
        (bytes_concat
          (bytes_concat (bytes_encode and_open_text) (print_bool_expr left))
          (bytes_encode separator_text))
        (bytes_concat (print_bool_expr right) (bytes_encode close_text))
  }

pub fn format_bool_expr (s : Source) : Result ParseError Bytes =
  match parse_bool_expr s Zero ((proof zero_left for LessEqNat) (source_length s)) {
    Parsed syntax consumed next ↦ Ok ParseError Bytes (print_bool_expr (erase_spans syntax));
    Failed err ↦ Err ParseError Bytes err
  }

theorem map_uint8_codes_retract
      (xs : List UInt8)
    : Equal (List UInt8) (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int xs)) xs =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      trans
        (List UInt8)
        (Cons
          UInt8
          (int_to_uint8_raw (uint8_to_int h))
          (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t)))
        (Cons UInt8 h (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t)))
        (Cons UInt8 h t)
        (cong
          UInt8
          (List UInt8)
          (int_to_uint8_raw (uint8_to_int h))
          h
          (λhead.
            Cons UInt8 head (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t)))
          (uint8_int_retract h))
        (cong
          (List UInt8)
          (List UInt8)
          (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t))
          t
          (Cons UInt8 h)
          (map_uint8_codes_retract t))
  }

theorem ascii_encoded_byte_view
      (s : String) (ascii : AllAscii s)
    : Equal Bytes
        (bytes_encode s)
        (list_to_bytes
          (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s)))) =
  bytes_to_list_injective
    (bytes_encode s)
    (list_to_bytes
      (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s))))
    (trans
      (List UInt8)
      (bytes_to_list (bytes_encode s))
      (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s)))
      (bytes_to_list
        (list_to_bytes
          (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s)))))
      (trans
        (List UInt8)
        (bytes_to_list (bytes_encode s))
        (map
          Int
          UInt8
          int_to_uint8_raw
          (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode s))))
        (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s)))
        (sym
          (List UInt8)
          (map
            Int
            UInt8
            int_to_uint8_raw
            (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode s))))
          (bytes_to_list (bytes_encode s))
          (map_uint8_codes_retract (bytes_to_list (bytes_encode s))))
        (cong
          (List Int)
          (List UInt8)
          (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode s)))
          (map Char Int charToInt (string_to_list_char s))
          (map Int UInt8 int_to_uint8_raw)
          (bytes_encode_ascii_octets s ascii)))
      (sym
        (List UInt8)
        (bytes_to_list
          (list_to_bytes
            (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s)))))
        (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s)))
        (list_bytes_roundtrip
          (map Int UInt8 int_to_uint8_raw (map Char Int charToInt (string_to_list_char s))))))

fn ascii_encoded_byte_codes (s : String) (ascii : AllAscii s) : AsciiBytes (bytes_encode s) =
  J
    (λcodes _. AllAsciiCodes codes)
    ascii
    (sym
      (List Int)
      (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode s)))
      (map Char Int charToInt (string_to_list_char s))
      (bytes_encode_ascii_octets s ascii))

theorem ascii_encoded_utf8 (s : String) (ascii : AllAscii s) : IsUtf8 (bytes_encode s) =
  ascii_bytes_utf8 (bytes_encode s) (ascii_encoded_byte_codes s ascii)

theorem map_appends
      (a : Type) (b : Type) (f : a → b) (xs : List a) (ys : List a)
    : Equal
        (List b)
        (map a b f (list_append a xs ys))
        (list_append b (map a b f xs) (map a b f ys)) =
  match xs {
    Nil ↦ Refl;
    Cons head tail ↦
      cong
        (List b)
        (List b)
        (map a b f (list_append a tail ys))
        (list_append b (map a b f tail) (map a b f ys))
        (Cons b (f head))
        (map_appends a b f tail ys)
  }

fn all_ascii_codes_append
      (xs : List Int) (ys : List Int) (left : AllAsciiCodes xs) (right : AllAsciiCodes ys)
    : AllAsciiCodes (list_append Int xs ys) =
  match left {
    NoCodes ↦ right;
    SomeCodes code tail witness rest ↦
      SomeCodes
        code
        (list_append Int tail ys)
        witness
        (all_ascii_codes_append tail ys rest right)
  }

fn ascii_bytes_concat
      (a : Bytes) (b : Bytes) (left : AsciiBytes a) (right : AsciiBytes b)
    : AsciiBytes (bytes_concat a b) =
  let
    concatenated_codes : AllAsciiCodes
      (list_append
        Int
        (map UInt8 Int uint8_to_int (bytes_to_list a))
        (map UInt8 Int uint8_to_int (bytes_to_list b))) =
      all_ascii_codes_append
        (map UInt8 Int uint8_to_int (bytes_to_list a))
        (map UInt8 Int uint8_to_int (bytes_to_list b))
        left
        right;
    mapped_codes : AllAsciiCodes
      (map UInt8 Int uint8_to_int (list_append UInt8 (bytes_to_list a) (bytes_to_list b))) =
      J
        (λcodes _. AllAsciiCodes codes)
        concatenated_codes
        (sym
          (List Int)
          (map UInt8 Int uint8_to_int (list_append UInt8 (bytes_to_list a) (bytes_to_list b)))
          (list_append
            Int
            (map UInt8 Int uint8_to_int (bytes_to_list a))
            (map UInt8 Int uint8_to_int (bytes_to_list b)))
          (map_appends UInt8 Int uint8_to_int (bytes_to_list a) (bytes_to_list b)))
  in
    J
      (λview _. AllAsciiCodes (map UInt8 Int uint8_to_int view))
      mapped_codes
      (sym
        (List UInt8)
        (bytes_to_list (bytes_concat a b))
        (list_append UInt8 (bytes_to_list a) (bytes_to_list b))
        (bytes_concat_list_view a b))

theorem ascii_concat_utf8
      (a : Bytes) (b : Bytes) (left : AsciiBytes a) (right : AsciiBytes b)
    : IsUtf8 (bytes_concat a b) =
  ascii_bytes_utf8 (bytes_concat a b) (ascii_bytes_concat a b left right)

const true_token_ascii : AllAscii true_token_text =
  SomeCodes
    116
    (Cons Int 114 (Cons Int 117 (Cons Int 101 (Nil Int))))
    (MkAsciiCode 116 Ascii116 Proved)
    (SomeCodes
      114
      (Cons Int 117 (Cons Int 101 (Nil Int)))
      (MkAsciiCode 114 Ascii114 Proved)
      (SomeCodes
        117
        (Cons Int 101 (Nil Int))
        (MkAsciiCode 117 Ascii117 Proved)
        (SomeCodes 101 (Nil Int) (MkAsciiCode 101 Ascii101 Proved) NoCodes)))

const false_token_ascii : AllAscii false_token_text =
  SomeCodes
    102
    (Cons Int 97 (Cons Int 108 (Cons Int 115 (Cons Int 101 (Nil Int)))))
    (MkAsciiCode 102 Ascii102 Proved)
    (SomeCodes
      97
      (Cons Int 108 (Cons Int 115 (Cons Int 101 (Nil Int))))
      (MkAsciiCode 97 Ascii97 Proved)
      (SomeCodes
        108
        (Cons Int 115 (Cons Int 101 (Nil Int)))
        (MkAsciiCode 108 Ascii108 Proved)
        (SomeCodes
          115
          (Cons Int 101 (Nil Int))
          (MkAsciiCode 115 Ascii115 Proved)
          (SomeCodes 101 (Nil Int) (MkAsciiCode 101 Ascii101 Proved) NoCodes))))

const not_open_ascii : AllAscii not_open_text =
  SomeCodes
    40
    (Cons Int 110 (Cons Int 111 (Cons Int 116 (Cons Int 32 (Nil Int)))))
    (MkAsciiCode 40 Ascii40 Proved)
    (SomeCodes
      110
      (Cons Int 111 (Cons Int 116 (Cons Int 32 (Nil Int))))
      (MkAsciiCode 110 Ascii110 Proved)
      (SomeCodes
        111
        (Cons Int 116 (Cons Int 32 (Nil Int)))
        (MkAsciiCode 111 Ascii111 Proved)
        (SomeCodes
          116
          (Cons Int 32 (Nil Int))
          (MkAsciiCode 116 Ascii116 Proved)
          (SomeCodes 32 (Nil Int) (MkAsciiCode 32 Ascii32 Proved) NoCodes))))

const and_open_ascii : AllAscii and_open_text =
  SomeCodes
    40
    (Cons Int 97 (Cons Int 110 (Cons Int 100 (Cons Int 32 (Nil Int)))))
    (MkAsciiCode 40 Ascii40 Proved)
    (SomeCodes
      97
      (Cons Int 110 (Cons Int 100 (Cons Int 32 (Nil Int))))
      (MkAsciiCode 97 Ascii97 Proved)
      (SomeCodes
        110
        (Cons Int 100 (Cons Int 32 (Nil Int)))
        (MkAsciiCode 110 Ascii110 Proved)
        (SomeCodes
          100
          (Cons Int 32 (Nil Int))
          (MkAsciiCode 100 Ascii100 Proved)
          (SomeCodes 32 (Nil Int) (MkAsciiCode 32 Ascii32 Proved) NoCodes))))

const separator_ascii : AllAscii separator_text =
  SomeCodes 32 (Nil Int) (MkAsciiCode 32 Ascii32 Proved) NoCodes

const close_ascii : AllAscii close_text =
  SomeCodes 41 (Nil Int) (MkAsciiCode 41 Ascii41 Proved) NoCodes

fn print_bool_expr_ascii (e : BoolExpr) : AsciiBytes (print_bool_expr e) =
  match e {
    BTrue ↦ ascii_encoded_byte_codes true_token_text true_token_ascii;
    BFalse ↦ ascii_encoded_byte_codes false_token_text false_token_ascii;
    BNot child ↦
      let
        open_ascii : AsciiBytes (bytes_encode not_open_text) =
          ascii_encoded_byte_codes not_open_text not_open_ascii;
        child_ascii : AsciiBytes (print_bool_expr child) = print_bool_expr_ascii child;
        open_child_ascii : AsciiBytes
          (bytes_concat (bytes_encode not_open_text) (print_bool_expr child)) =
          ascii_bytes_concat
            (bytes_encode not_open_text)
            (print_bool_expr child)
            open_ascii
            child_ascii;
        end_ascii : AsciiBytes (bytes_encode close_text) =
          ascii_encoded_byte_codes close_text close_ascii
      in
        ascii_bytes_concat
          (bytes_concat (bytes_encode not_open_text) (print_bool_expr child))
          (bytes_encode close_text)
          open_child_ascii
          end_ascii;
    BAnd left right ↦
      let
        open_ascii : AsciiBytes (bytes_encode and_open_text) =
          ascii_encoded_byte_codes and_open_text and_open_ascii;
        left_ascii : AsciiBytes (print_bool_expr left) = print_bool_expr_ascii left;
        open_left_ascii : AsciiBytes
          (bytes_concat (bytes_encode and_open_text) (print_bool_expr left)) =
          ascii_bytes_concat
            (bytes_encode and_open_text)
            (print_bool_expr left)
            open_ascii
            left_ascii;
        separator_bytes_ascii : AsciiBytes (bytes_encode separator_text) =
          ascii_encoded_byte_codes separator_text separator_ascii;
        prefix_ascii : AsciiBytes
          (bytes_concat
            (bytes_concat (bytes_encode and_open_text) (print_bool_expr left))
            (bytes_encode separator_text)) =
          ascii_bytes_concat
            (bytes_concat (bytes_encode and_open_text) (print_bool_expr left))
            (bytes_encode separator_text)
            open_left_ascii
            separator_bytes_ascii;
        right_ascii : AsciiBytes (print_bool_expr right) = print_bool_expr_ascii right;
        close_bytes_ascii : AsciiBytes (bytes_encode close_text) =
          ascii_encoded_byte_codes close_text close_ascii;
        suffix_ascii : AsciiBytes
          (bytes_concat (print_bool_expr right) (bytes_encode close_text)) =
          ascii_bytes_concat
            (print_bool_expr right)
            (bytes_encode close_text)
            right_ascii
            close_bytes_ascii
      in
        ascii_bytes_concat
          (bytes_concat
            (bytes_concat (bytes_encode and_open_text) (print_bool_expr left))
            (bytes_encode separator_text))
          (bytes_concat (print_bool_expr right) (bytes_encode close_text))
          prefix_ascii
          suffix_ascii
  }

pub theorem print_bool_expr_utf8 (e : BoolExpr) : IsUtf8 (print_bool_expr e) =
  ascii_bytes_utf8 (print_bool_expr e) (print_bool_expr_ascii e)

pub fn ParsedPrintedBool (e : BoolExpr) (outcome : ParseResult (Syntax BoolExpr)) : Prop =
  match outcome {
    Parsed syntax consumed next ↦ Equal BoolExpr (erase_spans syntax) e;
    Failed err ↦ Bottom
  }

pub theorem parse_bool_expr_print_round_trip
      (s : Source)
      (e : BoolExpr)
      (h : LessEqNat Zero (source_length s))
      (source_is_printed : Equal Bytes (source_bytes s) (print_bool_expr e))
    : ParsedPrintedBool e (parse_bool_expr s Zero h) =
  let
    cur : ByteCursor = MkByteCursor s Zero;
    expected : ParseResult (Syntax BoolExpr) =
      Parsed
        (Syntax BoolExpr)
        (printed_syntax cur e)
        (MkSpan Zero (byte_cursor_position (printed_end cur e)))
        (byte_cursor_position (printed_end cur e));
    parsed : Equal (ParseResult (Syntax BoolExpr)) (parse_bool_expr s Zero h) expected =
      parse_bool_expr_succeeds_printed_source s e h source_is_printed
  in
    J
      (λoutcome _. ParsedPrintedBool e outcome)
      (printed_syntax_erases e cur)
      (sym (ParseResult (Syntax BoolExpr)) (parse_bool_expr s Zero h) expected parsed)

pub theorem format_bool_expr_print_round_trip
      (s : Source)
      (e : BoolExpr)
      (source_is_printed : Equal Bytes (source_bytes s) (print_bool_expr e))
    : Equal
        (Result ParseError Bytes)
        (format_bool_expr s)
        (Ok ParseError Bytes (print_bool_expr e)) =
  let
    cur : ByteCursor = MkByteCursor s Zero;
    zero_bound : LessEqNat Zero (source_length s) =
      (proof zero_left for LessEqNat) (source_length s);
    parsed_result : ParseResult (Syntax BoolExpr) =
      Parsed
        (Syntax BoolExpr)
        (printed_syntax cur e)
        (MkSpan Zero (byte_cursor_position (printed_end cur e)))
        (byte_cursor_position (printed_end cur e));
    parsed : Equal
      (ParseResult (Syntax BoolExpr))
      (parse_bool_expr s Zero zero_bound)
      parsed_result =
      parse_bool_expr_succeeds_printed_source s e zero_bound source_is_printed;
    formatter_pointwise : Equal
      (Result ParseError Bytes)
      (format_bool_expr s)
      (format_bool_parse_outcome (parse_bool_expr s Zero zero_bound)) =
      Refl;
    formatted_parsed : Equal
      (Result ParseError Bytes)
      (format_bool_parse_outcome (parse_bool_expr s Zero zero_bound))
      (format_bool_parse_outcome parsed_result) =
      cong
        (ParseResult (Syntax BoolExpr))
        (Result ParseError Bytes)
        (parse_bool_expr s Zero zero_bound)
        parsed_result
        format_bool_parse_outcome
        parsed;
    printed_again : Equal Bytes
      (print_bool_expr (erase_spans (printed_syntax cur e)))
      (print_bool_expr e) =
      cong
        BoolExpr
        Bytes
        (erase_spans (printed_syntax cur e))
        e
        print_bool_expr
        (printed_syntax_erases e cur);
    formatted_value : Equal
      (Result ParseError Bytes)
      (format_bool_parse_outcome parsed_result)
      (Ok ParseError Bytes (print_bool_expr e)) =
      cong
        Bytes
        (Result ParseError Bytes)
        (print_bool_expr (erase_spans (printed_syntax cur e)))
        (print_bool_expr e)
        (λbytes. Ok ParseError Bytes bytes)
        printed_again
  in
    trans
      (Result ParseError Bytes)
      (format_bool_expr s)
      (format_bool_parse_outcome (parse_bool_expr s Zero zero_bound))
      (Ok ParseError Bytes (print_bool_expr e))
      formatter_pointwise
      (trans
        (Result ParseError Bytes)
        (format_bool_parse_outcome (parse_bool_expr s Zero zero_bound))
        (format_bool_parse_outcome parsed_result)
        (Ok ParseError Bytes (print_bool_expr e))
        formatted_parsed
        formatted_value)

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
      (leq_nat_successor_bound position))
    (byte_cursor_peek_in_bounds s position value peeked)

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
```

#### Generic bounded-parser laws

The public law follows from the decoder's checked bound. `parser_pure` returns
its input cursor unchanged; `parser_fail` locates its error at that cursor.
The private cursor and outcome bridges above establish the premises without
adding trust.

```ken
pub theorem parser_from_decoder_laws
      (a : Type)
      (decoder : Decoder ByteCursor Span a)
      (bounded : DecoderPreservesBounded a decoder)
    : ParserLaws a (parser_from_decoder a decoder) =
  let valid : ParserValid a (parser_from_decoder a decoder) =
    parser_from_decoder_valid_if_bounded a decoder bounded
  in
    and_intro
      (ParserValid a (parser_from_decoder a decoder))
      (And
        (ParserTotal a (parser_from_decoder a decoder))
        (ParserSourceLocal a (parser_from_decoder a decoder)))
      valid
      (and_intro
        (ParserTotal a (parser_from_decoder a decoder))
        (ParserSourceLocal a (parser_from_decoder a decoder))
        (λs.
          λstart.
            λh.
              match parser_from_decoder a decoder s start h {
                Parsed value consumed next ↦ Proved;
                Failed err ↦ Proved
              })
        (parser_valid_source_local a (parser_from_decoder a decoder) valid))

pub theorem parser_pure_laws (a : Type) (value : a) : ParserLaws a (parser_pure a value) =
  parser_from_decoder_laws
    a
    (decoder_pure ByteCursor Span a value)
    (λs. λstart. λcur. λsafe. safe)

pub theorem parser_fail_laws (a : Type) : ParserLaws a (parser_fail a) =
  parser_from_decoder_laws
    a
    (decoder_fail ByteCursor UInt8 Span a byte_cursor_ops)
    (λs. λstart. λcur. λsafe. byte_cursor_bounded_locate s start cur safe)
```

The bounded true twin uses a closed decoder that returns its input cursor. The
unbounded false twin returns `Suc (source_length s)` instead; its attempted
`ParserLaws` proof must fail at the end-position bound, not at name resolution.

```ken example
theorem bounded_parser_laws_true_twin
    : ParserLaws Bool (parser_from_decoder Bool (decoder_pure ByteCursor Span Bool True)) =
  parser_from_decoder_laws
    Bool
    (decoder_pure ByteCursor Span Bool True)
    (λs. λstart. λcur. λsafe. safe)
```

```ken reject
fn unbounded_parser_laws_decoder (cur : ByteCursor) : DecoderResult ByteCursor Span Bool =
  match cur {
    MkByteCursor s start ↦
      Decoded ByteCursor Span Bool True (MkByteCursor s (Suc (source_length s)))
  }

theorem unbounded_parser_laws_false_twin
    : ParserLaws Bool (parser_from_decoder Bool unbounded_parser_laws_decoder) =
  let valid : ParserValid Bool (parser_from_decoder Bool unbounded_parser_laws_decoder) =
    λs.
      λstart.
        λh.
          and_intro
            (ValidSpan s (MkSpan start (Suc (source_length s))))
            (And
              (Equal Nat start start)
              (Equal Nat (Suc (source_length s)) (Suc (source_length s))))
            (and_intro
              (LessEqNat start (Suc (source_length s)))
              (LessEqNat (Suc (source_length s)) (source_length s))
              ((proof trans for leq_nat)
                start
                (source_length s)
                (Suc (source_length s))
                h
                (leq_nat_successor_bound (source_length s)))
              Proved)
            (and_intro
              (Equal Nat start start)
              (Equal Nat (Suc (source_length s)) (Suc (source_length s)))
              Refl
              Refl)
  in
    and_intro
      (ParserValid Bool (parser_from_decoder Bool unbounded_parser_laws_decoder))
      (And
        (ParserTotal Bool (parser_from_decoder Bool unbounded_parser_laws_decoder))
        (ParserSourceLocal Bool (parser_from_decoder Bool unbounded_parser_laws_decoder)))
      valid
      (and_intro
        (ParserTotal Bool (parser_from_decoder Bool unbounded_parser_laws_decoder))
        (ParserSourceLocal Bool (parser_from_decoder Bool unbounded_parser_laws_decoder))
        (λs. λstart. λh. Proved)
        (parser_valid_source_local
          Bool
          (parser_from_decoder Bool unbounded_parser_laws_decoder)
          valid))
```

```ken
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
  parser_from_decoder_laws (Syntax BoolExpr) complete_bool_decoder complete_bool_decoder_bounded

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

### 4.4 Round-trip proof internals

```ken
fn ListNonempty (a : Type) (xs : List a) : Prop =
  match xs {
    Nil ↦ Bottom;
    Cons head tail ↦ Top
  }

theorem list_nonempty_from_code_head
      (xs : List UInt8) (code : Int) (rest : List Int)
    : Equal (List Int) (map UInt8 Int uint8_to_int xs) (Cons Int code rest)
      → ListNonempty UInt8 xs =
  match xs {
    Nil ↦ λcodes. absurd codes;
    Cons byte tail ↦ λcodes. Proved
  }

theorem true_encoded_nonempty
    : ListNonempty UInt8 (bytes_to_list (bytes_encode true_token_text)) =
  list_nonempty_from_code_head
    (bytes_to_list (bytes_encode true_token_text))
    116
    (Cons Int 114 (Cons Int 117 (Cons Int 101 (Nil Int))))
    (bytes_encode_ascii_octets true_token_text true_token_ascii)

theorem false_encoded_nonempty
    : ListNonempty UInt8 (bytes_to_list (bytes_encode false_token_text)) =
  list_nonempty_from_code_head
    (bytes_to_list (bytes_encode false_token_text))
    102
    (Cons Int 97 (Cons Int 108 (Cons Int 115 (Cons Int 101 (Nil Int)))))
    (bytes_encode_ascii_octets false_token_text false_token_ascii)

theorem not_open_encoded_nonempty
    : ListNonempty UInt8 (bytes_to_list (bytes_encode not_open_text)) =
  list_nonempty_from_code_head
    (bytes_to_list (bytes_encode not_open_text))
    40
    (Cons Int 110 (Cons Int 111 (Cons Int 116 (Cons Int 32 (Nil Int)))))
    (bytes_encode_ascii_octets not_open_text not_open_ascii)

theorem and_open_encoded_nonempty
    : ListNonempty UInt8 (bytes_to_list (bytes_encode and_open_text)) =
  list_nonempty_from_code_head
    (bytes_to_list (bytes_encode and_open_text))
    40
    (Cons Int 97 (Cons Int 110 (Cons Int 100 (Cons Int 32 (Nil Int)))))
    (bytes_encode_ascii_octets and_open_text and_open_ascii)

theorem list_nonempty_append_left
      (a : Type) (left : List a) (right : List a)
    : ListNonempty a left → ListNonempty a (list_append a left right) =
  match left {
    Nil ↦ λnonempty. absurd nonempty;
    Cons head tail ↦ λnonempty. Proved
  }

theorem bytes_concat_nonempty_left
      (left : Bytes) (right : Bytes) (nonempty : ListNonempty UInt8 (bytes_to_list left))
    : ListNonempty UInt8 (bytes_to_list (bytes_concat left right)) =
  J
    (λview _. ListNonempty UInt8 view)
    (list_nonempty_append_left UInt8 (bytes_to_list left) (bytes_to_list right) nonempty)
    (sym
      (List UInt8)
      (bytes_to_list (bytes_concat left right))
      (list_append UInt8 (bytes_to_list left) (bytes_to_list right))
      (bytes_concat_list_view left right))

theorem print_bool_expr_nonempty
      (e : BoolExpr)
    : ListNonempty UInt8 (bytes_to_list (print_bool_expr e)) =
  match e {
    BTrue ↦ true_encoded_nonempty;
    BFalse ↦ false_encoded_nonempty;
    BNot child ↦
      bytes_concat_nonempty_left
        (bytes_concat (bytes_encode not_open_text) (print_bool_expr child))
        (bytes_encode close_text)
        (bytes_concat_nonempty_left
          (bytes_encode not_open_text)
          (print_bool_expr child)
          not_open_encoded_nonempty);
    BAnd left right ↦
      bytes_concat_nonempty_left
        (bytes_concat
          (bytes_concat (bytes_encode and_open_text) (print_bool_expr left))
          (bytes_encode separator_text))
        (bytes_concat (print_bool_expr right) (bytes_encode close_text))
        (bytes_concat_nonempty_left
          (bytes_concat (bytes_encode and_open_text) (print_bool_expr left))
          (bytes_encode separator_text)
          (bytes_concat_nonempty_left
            (bytes_encode and_open_text)
            (print_bool_expr left)
            and_open_encoded_nonempty))
  }

theorem separator_encoded_nonempty
    : ListNonempty UInt8 (bytes_to_list (bytes_encode separator_text)) =
  list_nonempty_from_code_head
    (bytes_to_list (bytes_encode separator_text))
    32
    (Nil Int)
    (bytes_encode_ascii_octets separator_text separator_ascii)

theorem close_encoded_nonempty : ListNonempty UInt8 (bytes_to_list (bytes_encode close_text)) =
  list_nonempty_from_code_head
    (bytes_to_list (bytes_encode close_text))
    41
    (Nil Int)
    (bytes_encode_ascii_octets close_text close_ascii)

fn list_drop_at (a : Type) (n : Nat) (xs : List a) : List a =
  match n {
    Zero ↦ xs;
    Suc n2 ↦
      match xs {
        Nil ↦ Nil a;
        Cons h rest ↦ list_drop_at a n2 rest
      }
  }

fn list_tail_or_nil (a : Type) (xs : List a) : List a =
  match xs {
    Nil ↦ Nil a;
    Cons h rest ↦ rest
  }

theorem list_drop_at_suc
      (a : Type) (n : Nat) (xs : List a)
    : Equal (List a) (list_drop_at a (Suc n) xs) (list_tail_or_nil a (list_drop_at a n xs)) =
  match n {
    Zero ↦
      match xs {
        Nil ↦ Proved;
        Cons h rest ↦ Refl
      };
    Suc n2 ↦
      match xs {
        Nil ↦ Proved;
        Cons h rest ↦ list_drop_at_suc a n2 rest
      }
  }

theorem list_drop_at_nth
      (a : Type) (n : Nat) (xs : List a)
    : Equal (Option a) (nth a n xs) (nth a Zero (list_drop_at a n xs)) =
  match n {
    Zero ↦ Refl;
    Suc n2 ↦
      match xs {
        Nil ↦ Proved;
        Cons h rest ↦ list_drop_at_nth a n2 rest
      }
  }

theorem list_drop_at_length
      (a : Type) (n : Nat) (xs : List a)
    : Equal Nat (length a (list_drop_at a n xs)) (sub (length a xs) n) =
  match n {
    Zero ↦ Refl;
    Suc n2 ↦
      match xs {
        Nil ↦ Proved;
        Cons h rest ↦ list_drop_at_length a n2 rest
      }
  }

fn list_advance_over (a : Type) (start : Nat) (xs : List a) : Nat =
  match xs {
    Nil ↦ start;
    Cons h rest ↦ list_advance_over a (Suc start) rest
  }

theorem list_advance_over_append
      (a : Type) (start : Nat) (left : List a) (right : List a)
    : Equal Nat
        (list_advance_over a start (list_append a left right))
        (list_advance_over a (list_advance_over a start left) right) =
  match left {
    Nil ↦ Refl;
    Cons head tail ↦ list_advance_over_append a (Suc start) tail right
  }

theorem list_drop_at_advance_over
      (a : Type) (start : Nat) (source : List a) (suffix : List a) (prefix : List a)
    : (starts_with : Equal (List a) (list_drop_at a start source) (list_append a prefix suffix))
      → Equal (List a) (list_drop_at a (list_advance_over a start prefix) source) suffix =
  match prefix {
    Nil ↦ λstarts_with. starts_with;
    Cons h rest ↦
      λstarts_with.
        let next_starts_with : Equal
          (List a)
          (list_drop_at a (Suc start) source)
          (list_append a rest suffix) =
          trans
            (List a)
            (list_drop_at a (Suc start) source)
            (list_tail_or_nil a (list_drop_at a start source))
            (list_append a rest suffix)
            (list_drop_at_suc a start source)
            (cong
              (List a)
              (List a)
              (list_drop_at a start source)
              (list_append a (Cons a h rest) suffix)
              (list_tail_or_nil a)
              starts_with)
        in
          list_drop_at_advance_over a (Suc start) source suffix rest next_starts_with
  }

fn source_suffix (cur : ByteCursor) : List UInt8 =
  list_drop_at
    UInt8
    (byte_cursor_position cur)
    (bytes_to_list (source_bytes (byte_cursor_source cur)))

theorem source_suffix_length
      (cur : ByteCursor)
    : Equal Nat (length UInt8 (source_suffix cur)) (byte_cursor_remaining cur) =
  match cur {
    MkByteCursor source position ↦
      list_drop_at_length UInt8 position (bytes_to_list (source_bytes source))
  }

theorem source_suffix_positive
      (cur : ByteCursor)
      (head : UInt8)
      (rest : List UInt8)
      (starts_with : Equal (List UInt8) (source_suffix cur) (Cons UInt8 head rest))
    : Equal Bool (cursor_nat_lt Zero (byte_cursor_remaining cur)) True =
  trans
    Bool
    (cursor_nat_lt Zero (byte_cursor_remaining cur))
    (cursor_nat_lt Zero (length UInt8 (source_suffix cur)))
    True
    (cong
      Nat
      Bool
      (byte_cursor_remaining cur)
      (length UInt8 (source_suffix cur))
      (λn. cursor_nat_lt Zero n)
      (sym
        Nat
        (length UInt8 (source_suffix cur))
        (byte_cursor_remaining cur)
        (source_suffix_length cur)))
    (trans
      Bool
      (cursor_nat_lt Zero (length UInt8 (source_suffix cur)))
      (cursor_nat_lt Zero (length UInt8 (Cons UInt8 head rest)))
      True
      (cong
        (List UInt8)
        Bool
        (source_suffix cur)
        (Cons UInt8 head rest)
        (λxs. cursor_nat_lt Zero (length UInt8 xs))
        starts_with)
      Proved)

theorem cursor_nat_lt_from_leq_nat
      (left : Nat) (right : Nat)
    : Equal Bool (cursor_nat_lt left right) (leq_nat (Suc left) right) =
  match right {
    Zero ↦ Proved;
    Suc right2 ↦
      match left {
        Zero ↦ Proved;
        Suc left2 ↦ cursor_nat_lt_from_leq_nat left2 right2
      }
  }

theorem sub_positive_in_bounds
      (len : Nat)
    : (position : Nat)
      → Equal Bool (cursor_nat_lt Zero (sub len position)) True
      → LessEqNat (Suc position) len =
  match len {
    Zero ↦
      λposition.
        match position {
          Zero ↦ λpositive. absurd positive;
          Suc position2 ↦ λpositive. absurd positive
        };
    Suc len2 ↦
      λposition.
        match position {
          Zero ↦ λpositive. Proved;
          Suc position2 ↦ λpositive. sub_positive_in_bounds len2 position2 positive
        }
  }

theorem byte_cursor_advance_strict
      (cur : ByteCursor)
    : Equal Bool (cursor_nat_lt Zero (byte_cursor_remaining cur)) True
      → Equal Bool
        (cursor_nat_lt
          (byte_cursor_remaining (byte_cursor_advance cur))
          (byte_cursor_remaining cur))
        True =
  match cur {
    MkByteCursor source position ↦
      λpositive.
        trans
          Bool
          (cursor_nat_lt
            (sub (source_length source) (Suc position))
            (sub (source_length source) position))
          (leq_nat
            (Suc (sub (source_length source) (Suc position)))
            (sub (source_length source) position))
          True
          (cursor_nat_lt_from_leq_nat
            (sub (source_length source) (Suc position))
            (sub (source_length source) position))
          ((proof suc_decreases for sub)
            (source_length source)
            position
            (sub_positive_in_bounds (source_length source) position positive))
  }

fn cursor_after_codes (cur : ByteCursor) (codes : List UInt8) : ByteCursor =
  MkByteCursor
    (byte_cursor_source cur)
    (list_advance_over UInt8 (byte_cursor_position cur) codes)

theorem cursor_after_codes_append
      (cur : ByteCursor) (left : List UInt8) (right : List UInt8)
    : Equal ByteCursor
        (cursor_after_codes cur (list_append UInt8 left right))
        (cursor_after_codes (cursor_after_codes cur left) right) =
  cong
    Nat
    ByteCursor
    (list_advance_over UInt8 (byte_cursor_position cur) (list_append UInt8 left right))
    (list_advance_over UInt8 (list_advance_over UInt8 (byte_cursor_position cur) left) right)
    (λposition. MkByteCursor (byte_cursor_source cur) position)
    (list_advance_over_append UInt8 (byte_cursor_position cur) left right)

theorem source_suffix_after_codes
      (cur : ByteCursor) (codes : List UInt8) (rest : List UInt8)
    : Equal (List UInt8) (source_suffix cur) (list_append UInt8 codes rest)
      → Equal (List UInt8) (source_suffix (cursor_after_codes cur codes)) rest =
  λstarts_with.
    list_drop_at_advance_over
      UInt8
      (byte_cursor_position cur)
      (bytes_to_list (source_bytes (byte_cursor_source cur)))
      rest
      codes
      starts_with

theorem cursor_nat_lt_trans
      (left : Nat)
      (middle : Nat)
      (right : Nat)
      (first : Equal Bool (cursor_nat_lt left middle) True)
      (second : Equal Bool (cursor_nat_lt middle right) True)
    : Equal Bool (cursor_nat_lt left right) True =
  let
    left_to_middle : Equal Bool (leq_nat (Suc left) middle) True =
      trans
        Bool
        (leq_nat (Suc left) middle)
        (cursor_nat_lt left middle)
        True
        (sym
          Bool
          (cursor_nat_lt left middle)
          (leq_nat (Suc left) middle)
          (cursor_nat_lt_from_leq_nat left middle))
        first;
    middle_to_right : Equal Bool (leq_nat (Suc middle) right) True =
      trans
        Bool
        (leq_nat (Suc middle) right)
        (cursor_nat_lt middle right)
        True
        (sym
          Bool
          (cursor_nat_lt middle right)
          (leq_nat (Suc middle) right)
          (cursor_nat_lt_from_leq_nat middle right))
        second;
    left_to_middle_suc : Equal Bool (leq_nat (Suc left) (Suc middle)) True =
      (proof trans for leq_nat)
        (Suc left)
        middle
        (Suc middle)
        left_to_middle
        (leq_nat_successor_bound middle);
    left_to_right : Equal Bool (leq_nat (Suc left) right) True =
      (proof trans for leq_nat) (Suc left) (Suc middle) right left_to_middle_suc middle_to_right
  in
    trans
      Bool
      (cursor_nat_lt left right)
      (leq_nat (Suc left) right)
      True
      (cursor_nat_lt_from_leq_nat left right)
      left_to_right

theorem cursor_after_codes_nonempty_strict
      (cur : ByteCursor) (first : UInt8) (more : List UInt8)
    : (rest : List UInt8)
      → Equal (List UInt8) (source_suffix cur) (list_append UInt8 (Cons UInt8 first more) rest)
      → Equal Bool
        (cursor_nat_lt
          (byte_cursor_remaining (cursor_after_codes cur (Cons UInt8 first more)))
          (byte_cursor_remaining cur))
        True =
  match more {
    Nil ↦
      λrest.
        λstarts_with.
          byte_cursor_advance_strict cur (source_suffix_positive cur first rest starts_with);
    Cons second tail ↦
      λrest.
        λstarts_with.
          let
            after_first : ByteCursor = byte_cursor_advance cur;
            first_strict : Equal Bool
              (cursor_nat_lt (byte_cursor_remaining after_first) (byte_cursor_remaining cur))
              True =
              byte_cursor_advance_strict
                cur
                (source_suffix_positive
                  cur
                  first
                  (list_append UInt8 (Cons UInt8 second tail) rest)
                  starts_with);
            rest_starts : Equal
              (List UInt8)
              (source_suffix after_first)
              (list_append UInt8 (Cons UInt8 second tail) rest) =
              source_suffix_after_codes
                cur
                (Cons UInt8 first (Nil UInt8))
                (list_append UInt8 (Cons UInt8 second tail) rest)
                starts_with;
            rest_strict : Equal Bool
              (cursor_nat_lt
                (byte_cursor_remaining
                  (cursor_after_codes after_first (Cons UInt8 second tail)))
                (byte_cursor_remaining after_first))
              True =
              cursor_after_codes_nonempty_strict after_first second tail rest rest_starts
          in
            cursor_nat_lt_trans
              (byte_cursor_remaining (cursor_after_codes after_first (Cons UInt8 second tail)))
              (byte_cursor_remaining after_first)
              (byte_cursor_remaining cur)
              rest_strict
              first_strict
  }

fn list_last_byte (xs : List UInt8) : UInt8 =
  match xs {
    Nil ↦ (0 : UInt8);
    Cons head rest ↦
      match rest {
        Nil ↦ head;
        Cons next tail ↦ list_last_byte rest
      }
  }

fn token_codes_decoder (codes : List Int) : Decoder ByteCursor Span UInt8 =
  match codes {
    Nil ↦ byte_code_decoder (0 : Int);
    Cons code more ↦
      match more {
        Nil ↦ byte_code_decoder code;
        Cons next rest ↦
          decoder_seq
            ByteCursor
            Span
            UInt8
            UInt8
            (byte_code_decoder code)
            (token_codes_decoder more)
      }
  }

const true_token_codes : List Int = map Char Int charToInt (string_to_list_char true_token_text)

theorem true_token_code_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (true_token_decoder cur)
        (token_codes_decoder true_token_codes cur) =
  Refl

theorem false_token_code_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (false_token_decoder cur)
        (token_codes_decoder
          (map Char Int charToInt (string_to_list_char false_token_text))
          cur) =
  Refl

theorem not_open_token_code_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (not_open_token_decoder cur)
        (token_codes_decoder (map Char Int charToInt (string_to_list_char not_open_text)) cur) =
  Refl

theorem and_open_token_code_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (and_open_token_decoder cur)
        (token_codes_decoder (map Char Int charToInt (string_to_list_char and_open_text)) cur) =
  Refl

const true_initial_code : Int = 116

const false_initial_code : Int = 102

const open_initial_code : Int = 40

const not_open_remaining_codes : List Int =
  Cons Int 110 (Cons Int 111 (Cons Int 116 (Cons Int 32 (Nil Int))))

const and_open_after_second_codes : List Int =
  Cons Int 110 (Cons Int 100 (Cons Int 32 (Nil Int)))

const not_second_code : Int = 110

const and_second_code : Int = 97

const separator_code : Int = 32

const close_code : Int = 41

theorem separator_code_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder separator_code cur)
        (token_codes_decoder
          (map Char Int charToInt (string_to_list_char separator_text))
          cur) =
  Refl

theorem close_code_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder close_code cur)
        (token_codes_decoder (map Char Int charToInt (string_to_list_char close_text)) cur) =
  Refl

fn decoder_seq_resume
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (second : Decoder c loc b)
      (first_outcome : DecoderResult c loc a)
    : DecoderResult c loc b =
  match first_outcome {
    Decoded value next ↦ second next;
    DecoderFailed err ↦ DecoderFailed c loc b err
  }

theorem decoder_seq_resume_equation
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
        (decoder_seq_resume c loc a b second (first cur)) =
  Refl

theorem decoder_seq_success
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc b)
      (cur : c)
      (value : a)
      (mid : c)
      (last : b)
      (end : c)
      (first_ok : Equal (DecoderResult c loc a) (first cur) (Decoded c loc a value mid))
      (second_ok : Equal (DecoderResult c loc b) (second mid) (Decoded c loc b last end))
    : Equal
        (DecoderResult c loc b)
        (decoder_seq c loc a b first second cur)
        (Decoded c loc b last end) =
  trans
    (DecoderResult c loc b)
    (decoder_seq c loc a b first second cur)
    (decoder_seq_resume c loc a b second (first cur))
    (Decoded c loc b last end)
    (decoder_seq_resume_equation c loc a b first second cur)
    (trans
      (DecoderResult c loc b)
      (decoder_seq_resume c loc a b second (first cur))
      (decoder_seq_resume c loc a b second (Decoded c loc a value mid))
      (Decoded c loc b last end)
      (cong
        (DecoderResult c loc a)
        (DecoderResult c loc b)
        (first cur)
        (Decoded c loc a value mid)
        (decoder_seq_resume c loc a b second)
        first_ok)
      second_ok)

theorem token_mapped_single
      (head : UInt8)
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (Cons UInt8 head (Nil UInt8)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (token_codes_decoder (map UInt8 Int uint8_to_int (Cons UInt8 head (Nil UInt8))) cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (Cons UInt8 head (Nil UInt8)))
          (cursor_after_codes cur (Cons UInt8 head (Nil UInt8)))) =
  byte_code_success_from_head cur (uint8_to_int head) head rest starts_with Refl

theorem token_mapped_succeeds_cons
      (tail : List UInt8)
    : (head : UInt8)
      → (cur : ByteCursor)
      → (rest : List UInt8)
      → Equal (List UInt8) (source_suffix cur) (list_append UInt8 (Cons UInt8 head tail) rest)
      → Equal
        (DecoderResult ByteCursor Span UInt8)
        (token_codes_decoder (map UInt8 Int uint8_to_int (Cons UInt8 head tail)) cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (Cons UInt8 head tail))
          (cursor_after_codes cur (Cons UInt8 head tail))) =
  match tail {
    Nil ↦ λhead. λcur. λrest. λstarts_with. token_mapped_single head cur rest starts_with;
    Cons second rest_bytes ↦
      λhead.
        λcur.
          λrest.
            λstarts_with.
              let
                remaining_bytes : List UInt8 =
                  list_append UInt8 (Cons UInt8 second rest_bytes) rest;
                first_ok : Equal
                  (DecoderResult ByteCursor Span UInt8)
                  (byte_code_decoder (uint8_to_int head) cur)
                  (Decoded ByteCursor Span UInt8 head (byte_cursor_advance cur)) =
                  byte_code_success_from_head
                    cur
                    (uint8_to_int head)
                    head
                    remaining_bytes
                    starts_with
                    Refl;
                after_first : ByteCursor = byte_cursor_advance cur;
                rest_starts : Equal
                  (List UInt8)
                  (source_suffix after_first)
                  (list_append UInt8 (Cons UInt8 second rest_bytes) rest) =
                  source_suffix_after_codes
                    cur
                    (Cons UInt8 head (Nil UInt8))
                    remaining_bytes
                    starts_with;
                rest_ok : Equal
                  (DecoderResult ByteCursor Span UInt8)
                  (token_codes_decoder
                    (map UInt8 Int uint8_to_int (Cons UInt8 second rest_bytes))
                    after_first)
                  (Decoded
                    ByteCursor
                    Span
                    UInt8
                    (list_last_byte (Cons UInt8 second rest_bytes))
                    (cursor_after_codes after_first (Cons UInt8 second rest_bytes))) =
                  token_mapped_succeeds_cons rest_bytes second after_first rest rest_starts
              in
                decoder_seq_success
                  ByteCursor
                  Span
                  UInt8
                  UInt8
                  (byte_code_decoder (uint8_to_int head))
                  (token_codes_decoder
                    (map UInt8 Int uint8_to_int (Cons UInt8 second rest_bytes)))
                  cur
                  head
                  after_first
                  (list_last_byte (Cons UInt8 second rest_bytes))
                  (cursor_after_codes after_first (Cons UInt8 second rest_bytes))
                  first_ok
                  rest_ok
  }

theorem token_mapped_succeeds
      (prefix : List UInt8)
    : (cur : ByteCursor)
      → (rest : List UInt8)
      → Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest)
      → ListNonempty UInt8 prefix
      → Equal
        (DecoderResult ByteCursor Span UInt8)
        (token_codes_decoder (map UInt8 Int uint8_to_int prefix) cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte prefix)
          (cursor_after_codes cur prefix)) =
  match prefix {
    Nil ↦ λcur. λrest. λstarts_with. λnonempty. absurd nonempty;
    Cons head tail ↦
      λcur.
        λrest.
          λstarts_with. λnonempty. token_mapped_succeeds_cons tail head cur rest starts_with
  }

theorem ascii_token_succeeds
      (s : String)
      (ascii : AllAscii s)
      (decoder : Decoder ByteCursor Span UInt8)
      (same_decoder : (cur : ByteCursor)
        → Equal
        (DecoderResult ByteCursor Span UInt8)
        (decoder cur)
        (token_codes_decoder (map Char Int charToInt (string_to_list_char s)) cur))
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode s)) rest))
      (nonempty : ListNonempty UInt8 (bytes_to_list (bytes_encode s)))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (decoder cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode s)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode s)))) =
  let
    encoded_bytes : List UInt8 = bytes_to_list (bytes_encode s);
    matched_codes : Equal
      (List Int)
      (map Char Int charToInt (string_to_list_char s))
      (map UInt8 Int uint8_to_int encoded_bytes) =
      sym
        (List Int)
        (map UInt8 Int uint8_to_int encoded_bytes)
        (map Char Int charToInt (string_to_list_char s))
        (bytes_encode_ascii_octets s ascii);
    matched_decoder : Equal
      (DecoderResult ByteCursor Span UInt8)
      (token_codes_decoder (map Char Int charToInt (string_to_list_char s)) cur)
      (token_codes_decoder (map UInt8 Int uint8_to_int encoded_bytes) cur) =
      cong
        (List Int)
        (DecoderResult ByteCursor Span UInt8)
        (map Char Int charToInt (string_to_list_char s))
        (map UInt8 Int uint8_to_int encoded_bytes)
        (λcodes. token_codes_decoder codes cur)
        matched_codes
  in
    trans
      (DecoderResult ByteCursor Span UInt8)
      (decoder cur)
      (token_codes_decoder (map UInt8 Int uint8_to_int encoded_bytes) cur)
      (Decoded
        ByteCursor
        Span
        UInt8
        (list_last_byte encoded_bytes)
        (cursor_after_codes cur encoded_bytes))
      (trans
        (DecoderResult ByteCursor Span UInt8)
        (decoder cur)
        (token_codes_decoder (map Char Int charToInt (string_to_list_char s)) cur)
        (token_codes_decoder (map UInt8 Int uint8_to_int encoded_bytes) cur)
        (same_decoder cur)
        matched_decoder)
      (token_mapped_succeeds encoded_bytes cur rest starts_with nonempty)

theorem true_token_succeeds
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode true_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (true_token_decoder cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode true_token_text)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text)))) =
  ascii_token_succeeds
    true_token_text
    true_token_ascii
    true_token_decoder
    true_token_code_decoder_pointwise
    cur
    rest
    starts_with
    true_encoded_nonempty

theorem false_token_succeeds
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode false_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (false_token_decoder cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode false_token_text)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode false_token_text)))) =
  ascii_token_succeeds
    false_token_text
    false_token_ascii
    false_token_decoder
    false_token_code_decoder_pointwise
    cur
    rest
    starts_with
    false_encoded_nonempty

theorem not_open_token_succeeds
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode not_open_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (not_open_token_decoder cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode not_open_text)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode not_open_text)))) =
  ascii_token_succeeds
    not_open_text
    not_open_ascii
    not_open_token_decoder
    not_open_token_code_decoder_pointwise
    cur
    rest
    starts_with
    not_open_encoded_nonempty

theorem and_open_token_succeeds
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode and_open_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (and_open_token_decoder cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode and_open_text)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode and_open_text)))) =
  ascii_token_succeeds
    and_open_text
    and_open_ascii
    and_open_token_decoder
    and_open_token_code_decoder_pointwise
    cur
    rest
    starts_with
    and_open_encoded_nonempty

theorem separator_byte_succeeds
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode separator_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder separator_code cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode separator_text)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode separator_text)))) =
  ascii_token_succeeds
    separator_text
    separator_ascii
    (byte_code_decoder separator_code)
    separator_code_decoder_pointwise
    cur
    rest
    starts_with
    separator_encoded_nonempty

theorem close_byte_succeeds
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode close_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder close_code cur)
        (Decoded
          ByteCursor
          Span
          UInt8
          (list_last_byte (bytes_to_list (bytes_encode close_text)))
          (cursor_after_codes cur (bytes_to_list (bytes_encode close_text)))) =
  ascii_token_succeeds
    close_text
    close_ascii
    (byte_code_decoder close_code)
    close_code_decoder_pointwise
    cur
    rest
    starts_with
    close_encoded_nonempty

fn bool_leaf_result
      (cur : ByteCursor) (value : BoolExpr) (token_result : DecoderResult ByteCursor Span UInt8)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  match token_result {
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
          value)
        next
  }

theorem bool_true_result_equation
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_true_decoder cur)
        (bool_leaf_result cur BTrue (true_token_decoder cur)) =
  Refl

theorem bool_false_result_equation
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_false_decoder cur)
        (bool_leaf_result cur BFalse (false_token_decoder cur)) =
  Refl

theorem bool_leaf_from_token
      (cur : ByteCursor)
      (value : BoolExpr)
      (token_decoder : Decoder ByteCursor Span UInt8)
      (last : UInt8)
      (next : ByteCursor)
      (token_succeeds : Equal
        (DecoderResult ByteCursor Span UInt8)
        (token_decoder cur)
        (Decoded ByteCursor Span UInt8 last next))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_leaf_result cur value (token_decoder cur))
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (syntax_leaf
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position next)
            value)
          next) =
  cong
    (DecoderResult ByteCursor Span UInt8)
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (token_decoder cur)
    (Decoded ByteCursor Span UInt8 last next)
    (bool_leaf_result cur value)
    token_succeeds

theorem bool_true_on_printed_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode true_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_true_decoder cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (syntax_leaf
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position
              (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text))))
            BTrue)
          (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text)))) =
  trans
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (bool_true_decoder cur)
    (bool_leaf_result cur BTrue (true_token_decoder cur))
    (Decoded
      ByteCursor
      Span
      (Syntax BoolExpr)
      (syntax_leaf
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position
          (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text))))
        BTrue)
      (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text))))
    (bool_true_result_equation cur)
    (bool_leaf_from_token
      cur
      BTrue
      true_token_decoder
      (list_last_byte (bytes_to_list (bytes_encode true_token_text)))
      (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text)))
      (true_token_succeeds cur rest starts_with))

theorem different_token_heads
      (equal : Equal Int false_initial_code true_initial_code)
    : Bottom =
  absurd equal

theorem int_eq_false_verdict
      (left : Int) (right : Int) (verdict : Bool)
    : Equal Bool verdict (eq_int left right)
      → (Equal Int left right → Bottom)
      → Equal Bool verdict False =
  match verdict {
    True ↦
      λsame.
        λdifferent.
          absurd (different (int_eq_sound left right (sym Bool True (eq_int left right) same)));
    False ↦ λsame. λdifferent. Proved
  }

theorem int_eq_false_from_neq
      (left : Int) (right : Int) (different : Equal Int left right → Bottom)
    : Equal Bool (eq_int left right) False =
  int_eq_false_verdict left right (eq_int left right) Refl different

theorem decoder_seq_first_rejected
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc b)
      (cur : c)
      (at : loc)
      (first_rejected : Equal
        (DecoderResult c loc a)
        (first cur)
        (DecoderFailed c loc a (DecoderRejected loc at)))
    : Equal
        (DecoderResult c loc b)
        (decoder_seq c loc a b first second cur)
        (DecoderFailed c loc b (DecoderRejected loc at)) =
  trans
    (DecoderResult c loc b)
    (decoder_seq c loc a b first second cur)
    (decoder_seq_resume c loc a b second (first cur))
    (DecoderFailed c loc b (DecoderRejected loc at))
    (decoder_seq_resume_equation c loc a b first second cur)
    (cong
      (DecoderResult c loc a)
      (DecoderResult c loc b)
      (first cur)
      (DecoderFailed c loc a (DecoderRejected loc at))
      (decoder_seq_resume c loc a b second)
      first_rejected)

const true_remaining_codes : List Int = Cons Int 114 (Cons Int 117 (Cons Int 101 (Nil Int)))

theorem true_token_rejected_by_initial
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (actual : Int)
      (actual_rest : List Int)
      (different : Equal Int actual true_initial_code → Bottom)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int actual actual_rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (true_token_decoder cur)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
  trans
    (DecoderResult ByteCursor Span UInt8)
    (true_token_decoder cur)
    (token_codes_decoder true_token_codes cur)
    (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur)))
    (true_token_code_decoder_pointwise cur)
    (decoder_seq_first_rejected
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder true_initial_code)
      (token_codes_decoder true_remaining_codes)
      cur
      (byte_cursor_locate cur)
      (byte_code_rejects_different_prefix
        cur
        true_initial_code
        actual
        actual_rest
        prefix
        rest
        different
        starts_with
        mapped))

const false_remaining_codes : List Int =
  Cons Int 97 (Cons Int 108 (Cons Int 115 (Cons Int 101 (Nil Int))))

theorem false_token_rejected_by_initial
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (actual : Int)
      (actual_rest : List Int)
      (different : Equal Int actual false_initial_code → Bottom)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int actual actual_rest))
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (false_token_decoder cur)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
  trans
    (DecoderResult ByteCursor Span UInt8)
    (false_token_decoder cur)
    (token_codes_decoder (map Char Int charToInt (string_to_list_char false_token_text)) cur)
    (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur)))
    (false_token_code_decoder_pointwise cur)
    (decoder_seq_first_rejected
      ByteCursor
      Span
      UInt8
      UInt8
      (byte_code_decoder false_initial_code)
      (token_codes_decoder false_remaining_codes)
      cur
      (byte_cursor_locate cur)
      (byte_code_rejects_different_prefix
        cur
        false_initial_code
        actual
        actual_rest
        prefix
        rest
        different
        starts_with
        mapped))

theorem bool_leaf_rejected_from_token
      (cur : ByteCursor)
      (value : BoolExpr)
      (token_decoder : Decoder ByteCursor Span UInt8)
      (token_rejected : Equal
        (DecoderResult ByteCursor Span UInt8)
        (token_decoder cur)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_leaf_result cur value (token_decoder cur))
        (DecoderFailed
          ByteCursor
          Span
          (Syntax BoolExpr)
          (DecoderRejected Span (byte_cursor_locate cur))) =
  cong
    (DecoderResult ByteCursor Span UInt8)
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (token_decoder cur)
    (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur)))
    (bool_leaf_result cur value)
    token_rejected

theorem bool_true_rejected_by_initial
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (actual : Int)
      (actual_rest : List Int)
      (different : Equal Int actual true_initial_code → Bottom)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int actual actual_rest))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_true_decoder cur)
        (DecoderFailed
          ByteCursor
          Span
          (Syntax BoolExpr)
          (DecoderRejected Span (byte_cursor_locate cur))) =
  trans
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (bool_true_decoder cur)
    (bool_leaf_result cur BTrue (true_token_decoder cur))
    (DecoderFailed
      ByteCursor
      Span
      (Syntax BoolExpr)
      (DecoderRejected Span (byte_cursor_locate cur)))
    (bool_true_result_equation cur)
    (bool_leaf_rejected_from_token
      cur
      BTrue
      true_token_decoder
      (true_token_rejected_by_initial
        cur
        prefix
        rest
        actual
        actual_rest
        different
        starts_with
        mapped))

theorem bool_false_rejected_by_initial
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (actual : Int)
      (actual_rest : List Int)
      (different : Equal Int actual false_initial_code → Bottom)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int actual actual_rest))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_false_decoder cur)
        (DecoderFailed
          ByteCursor
          Span
          (Syntax BoolExpr)
          (DecoderRejected Span (byte_cursor_locate cur))) =
  trans
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (bool_false_decoder cur)
    (bool_leaf_result cur BFalse (false_token_decoder cur))
    (DecoderFailed
      ByteCursor
      Span
      (Syntax BoolExpr)
      (DecoderRejected Span (byte_cursor_locate cur)))
    (bool_false_result_equation cur)
    (bool_leaf_rejected_from_token
      cur
      BFalse
      false_token_decoder
      (false_token_rejected_by_initial
        cur
        prefix
        rest
        actual
        actual_rest
        different
        starts_with
        mapped))

theorem bool_false_on_printed_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode false_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_false_decoder cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (syntax_leaf
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position
              (cursor_after_codes cur (bytes_to_list (bytes_encode false_token_text))))
            BFalse)
          (cursor_after_codes cur (bytes_to_list (bytes_encode false_token_text)))) =
  trans
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (bool_false_decoder cur)
    (bool_leaf_result cur BFalse (false_token_decoder cur))
    (Decoded
      ByteCursor
      Span
      (Syntax BoolExpr)
      (syntax_leaf
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position
          (cursor_after_codes cur (bytes_to_list (bytes_encode false_token_text))))
        BFalse)
      (cursor_after_codes cur (bytes_to_list (bytes_encode false_token_text))))
    (bool_false_result_equation cur)
    (bool_leaf_from_token
      cur
      BFalse
      false_token_decoder
      (list_last_byte (bytes_to_list (bytes_encode false_token_text)))
      (cursor_after_codes cur (bytes_to_list (bytes_encode false_token_text)))
      (false_token_succeeds cur rest starts_with))

fn decoder_alt_resume
      (c : Type)
      (loc : Type)
      (a : Type)
      (second : Decoder c loc a)
      (cur : c)
      (first_outcome : DecoderResult c loc a)
    : DecoderResult c loc a =
  decoder_alt c loc a (λignored. first_outcome) second cur

theorem decoder_alt_resume_equation
      (c : Type)
      (loc : Type)
      (a : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc a)
      (cur : c)
    : Equal
        (DecoderResult c loc a)
        (decoder_alt c loc a first second cur)
        (decoder_alt_resume c loc a second cur (first cur)) =
  Refl

theorem decoder_alt_first_success
      (c : Type)
      (loc : Type)
      (a : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc a)
      (cur : c)
      (value : a)
      (next : c)
      (succeeds : Equal (DecoderResult c loc a) (first cur) (Decoded c loc a value next))
    : Equal
        (DecoderResult c loc a)
        (decoder_alt c loc a first second cur)
        (Decoded c loc a value next) =
  trans
    (DecoderResult c loc a)
    (decoder_alt c loc a first second cur)
    (decoder_alt_resume c loc a second cur (first cur))
    (Decoded c loc a value next)
    (decoder_alt_resume_equation c loc a first second cur)
    (cong
      (DecoderResult c loc a)
      (DecoderResult c loc a)
      (first cur)
      (Decoded c loc a value next)
      (decoder_alt_resume c loc a second cur)
      succeeds)

fn bool_layer_last
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
    : Decoder ByteCursor Span (Syntax BoolExpr) =
  decoder_alt
    ByteCursor
    Span
    (Syntax BoolExpr)
    (bool_not_decoder recur)
    (bool_and_decoder recur)

fn bool_layer_rest
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
    : Decoder ByteCursor Span (Syntax BoolExpr) =
  decoder_alt ByteCursor Span (Syntax BoolExpr) bool_false_decoder (bool_layer_last recur)

theorem bool_layer_true_on_printed_token
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode true_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (syntax_leaf
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position
              (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text))))
            BTrue)
          (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text)))) =
  decoder_alt_first_success
    ByteCursor
    Span
    (Syntax BoolExpr)
    bool_true_decoder
    (bool_layer_rest recur)
    cur
    (syntax_leaf
      (byte_cursor_source cur)
      (byte_cursor_position cur)
      (byte_cursor_position
        (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text))))
      BTrue)
    (cursor_after_codes cur (bytes_to_list (bytes_encode true_token_text)))
    (bool_true_on_printed_token cur rest starts_with)

fn encoded_token_end (cur : ByteCursor) (text : String) : ByteCursor =
  cursor_after_codes cur (bytes_to_list (bytes_encode text))

fn encoded_token_syntax
      (cur : ByteCursor) (text : String) (value : BoolExpr)
    : Syntax BoolExpr =
  syntax_leaf
    (byte_cursor_source cur)
    (byte_cursor_position cur)
    (byte_cursor_position (encoded_token_end cur text))
    value

theorem false_token_code_view
    : Equal
        (List Int)
        (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode false_token_text)))
        (Cons Int false_initial_code false_remaining_codes) =
  bytes_encode_ascii_octets false_token_text false_token_ascii

theorem bool_layer_false_on_printed_token
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode false_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (encoded_token_syntax cur false_token_text BFalse)
          (encoded_token_end cur false_token_text)) =
  let
    false_prefix : List UInt8 = bytes_to_list (bytes_encode false_token_text);
    false_next : ByteCursor = encoded_token_end cur false_token_text;
    false_syntax : Syntax BoolExpr = encoded_token_syntax cur false_token_text BFalse;
    true_rejected : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_true_decoder cur)
      (DecoderFailed
        ByteCursor
        Span
        (Syntax BoolExpr)
        (DecoderRejected Span (byte_cursor_locate cur))) =
      bool_true_rejected_by_initial
        cur
        false_prefix
        rest
        false_initial_code
        false_remaining_codes
        different_token_heads
        starts_with
        false_token_code_view;
    false_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_layer_rest recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) false_syntax false_next) =
      decoder_alt_first_success
        ByteCursor
        Span
        (Syntax BoolExpr)
        bool_false_decoder
        (bool_layer_last recur)
        cur
        false_syntax
        false_next
        (bool_false_on_printed_token cur rest starts_with)
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_decoder_layer recur cur)
      (bool_layer_rest recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) false_syntax false_next)
      (decoder_alt_rejection_uses_second
        ByteCursor
        Span
        (Syntax BoolExpr)
        bool_true_decoder
        (bool_layer_rest recur)
        cur
        (byte_cursor_locate cur)
        true_rejected)
      false_succeeds

fn byte_code_result_decision
      (cur : ByteCursor) (code : Int) (head : UInt8) (decision : Bool)
    : DecoderResult ByteCursor Span UInt8 =
  match decision {
    True ↦ Decoded ByteCursor Span UInt8 head (byte_cursor_advance cur);
    False ↦ DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))
  }

fn byte_code_result_peek
      (cur : ByteCursor) (code : Int) (observed : Option UInt8)
    : DecoderResult ByteCursor Span UInt8 =
  match observed {
    None ↦ DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur));
    Some head ↦ byte_code_result_decision cur code head (eq_int (uint8_to_int head) code)
  }

theorem byte_code_result_peek_equation
      (cur : ByteCursor) (code : Int)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder code cur)
        (byte_code_result_peek cur code (byte_cursor_peek cur)) =
  Refl

theorem mapped_codes_cons_head
      (head : UInt8)
      (tail : List UInt8)
      (code : Int)
      (codes : List Int)
      (matches : Equal
        (List Int)
        (map UInt8 Int uint8_to_int (Cons UInt8 head tail))
        (Cons Int code codes))
    : Equal Int (uint8_to_int head) code =
  and_fst
    (Equal Int (uint8_to_int head) code)
    (Equal (List Int) (map UInt8 Int uint8_to_int tail) codes)
    matches

theorem mapped_codes_cons_tail
      (head : UInt8)
      (tail : List UInt8)
      (code : Int)
      (codes : List Int)
      (matches : Equal
        (List Int)
        (map UInt8 Int uint8_to_int (Cons UInt8 head tail))
        (Cons Int code codes))
    : Equal (List Int) (map UInt8 Int uint8_to_int tail) codes =
  and_snd
    (Equal Int (uint8_to_int head) code)
    (Equal (List Int) (map UInt8 Int uint8_to_int tail) codes)
    matches

theorem byte_code_decoded_self
      (c : Type) (loc : Type) (a : Type) (cur : c) (head : a)
    : Equal (DecoderResult c loc a) (Decoded c loc a head cur) (Decoded c loc a head cur) =
  Refl

theorem byte_code_decision_true_base
      (cur : ByteCursor) (code : Int) (head : UInt8)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_result_decision cur code head True)
        (Decoded ByteCursor Span UInt8 head (byte_cursor_advance cur)) =
  byte_code_decoded_self ByteCursor Span UInt8 (byte_cursor_advance cur) head

theorem byte_code_decision_true
      (cur : ByteCursor) (code : Int) (head : UInt8) (decision : Bool)
    : Equal Bool decision True
      → Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_result_decision cur code head decision)
        (Decoded ByteCursor Span UInt8 head (byte_cursor_advance cur)) =
  match decision {
    True ↦ λaccepted. byte_code_decision_true_base cur code head;
    False ↦ λaccepted. absurd accepted
  }

theorem byte_code_success_from_head
      (cur : ByteCursor)
      (code : Int)
      (head : UInt8)
      (rest : List UInt8)
      (starts_with : Equal (List UInt8) (source_suffix cur) (Cons UInt8 head rest))
      (matching : Equal Int (uint8_to_int head) code)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder code cur)
        (Decoded ByteCursor Span UInt8 head (byte_cursor_advance cur)) =
  let
    peeked_head : Equal (Option UInt8) (byte_cursor_peek cur) (Some UInt8 head) =
      trans
        (Option UInt8)
        (byte_cursor_peek cur)
        (nth UInt8 Zero (source_suffix cur))
        (Some UInt8 head)
        (list_drop_at_nth
          UInt8
          (byte_cursor_position cur)
          (bytes_to_list (source_bytes (byte_cursor_source cur))))
        (cong
          (List UInt8)
          (Option UInt8)
          (source_suffix cur)
          (Cons UInt8 head rest)
          (nth UInt8 Zero)
          starts_with);
    observed_head : Equal
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_result_peek cur code (byte_cursor_peek cur))
      (byte_code_result_peek cur code (Some UInt8 head)) =
      cong
        (Option UInt8)
        (DecoderResult ByteCursor Span UInt8)
        (byte_cursor_peek cur)
        (Some UInt8 head)
        (byte_code_result_peek cur code)
        peeked_head;
    accepted : Equal Bool (eq_int (uint8_to_int head) code) True =
      int_eq_complete (uint8_to_int head) code matching
  in
    trans
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_decoder code cur)
      (byte_code_result_peek cur code (Some UInt8 head))
      (Decoded ByteCursor Span UInt8 head (byte_cursor_advance cur))
      (trans
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder code cur)
        (byte_code_result_peek cur code (byte_cursor_peek cur))
        (byte_code_result_peek cur code (Some UInt8 head))
        (byte_code_result_peek_equation cur code)
        observed_head)
      (byte_code_decision_true cur code head (eq_int (uint8_to_int head) code) accepted)

theorem byte_code_failed_self
      (c : Type) (loc : Type) (a : Type) (err : DecoderError loc)
    : Equal (DecoderResult c loc a) (DecoderFailed c loc a err) (DecoderFailed c loc a err) =
  Refl

theorem byte_code_decision_false_base
      (cur : ByteCursor) (code : Int) (head : UInt8)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_result_decision cur code head False)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
  byte_code_failed_self ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))

theorem byte_code_decision_false
      (cur : ByteCursor) (code : Int) (head : UInt8) (decision : Bool)
    : Equal Bool decision False
      → Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_result_decision cur code head decision)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
  match decision {
    True ↦ λrejected. absurd rejected;
    False ↦ λrejected. byte_code_decision_false_base cur code head
  }

theorem byte_code_rejected_from_head
      (cur : ByteCursor)
      (code : Int)
      (head : UInt8)
      (rest : List UInt8)
      (starts_with : Equal (List UInt8) (source_suffix cur) (Cons UInt8 head rest))
      (different : Equal Int (uint8_to_int head) code → Bottom)
    : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder code cur)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
  let
    peeked_head : Equal (Option UInt8) (byte_cursor_peek cur) (Some UInt8 head) =
      trans
        (Option UInt8)
        (byte_cursor_peek cur)
        (nth UInt8 Zero (source_suffix cur))
        (Some UInt8 head)
        (list_drop_at_nth
          UInt8
          (byte_cursor_position cur)
          (bytes_to_list (source_bytes (byte_cursor_source cur))))
        (cong
          (List UInt8)
          (Option UInt8)
          (source_suffix cur)
          (Cons UInt8 head rest)
          (nth UInt8 Zero)
          starts_with);
    observed_head : Equal
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_result_peek cur code (byte_cursor_peek cur))
      (byte_code_result_peek cur code (Some UInt8 head)) =
      cong
        (Option UInt8)
        (DecoderResult ByteCursor Span UInt8)
        (byte_cursor_peek cur)
        (Some UInt8 head)
        (byte_code_result_peek cur code)
        peeked_head;
    rejected : Equal Bool (eq_int (uint8_to_int head) code) False =
      int_eq_false_from_neq (uint8_to_int head) code different
  in
    trans
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_decoder code cur)
      (byte_code_result_peek cur code (Some UInt8 head))
      (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur)))
      (trans
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder code cur)
        (byte_code_result_peek cur code (byte_cursor_peek cur))
        (byte_code_result_peek cur code (Some UInt8 head))
        (byte_code_result_peek_equation cur code)
        observed_head)
      (byte_code_decision_false cur code head (eq_int (uint8_to_int head) code) rejected)

theorem byte_code_rejects_different_prefix
      (cur : ByteCursor)
      (expected : Int)
      (actual : Int)
      (actual_rest : List Int)
      (prefix : List UInt8)
      (rest : List UInt8)
      (different : Equal Int actual expected → Bottom)
    : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest)
      → Equal (List Int) (map UInt8 Int uint8_to_int prefix) (Cons Int actual actual_rest)
      → Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder expected cur)
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
  match prefix {
    Nil ↦ λstarts_with. λmapped. absurd mapped;
    Cons head tail ↦
      λstarts_with.
        λmapped.
          let
            head_is_actual : Equal Int (uint8_to_int head) actual =
              mapped_codes_cons_head head tail actual actual_rest mapped;
            head_is_different : Equal Int (uint8_to_int head) expected → Bottom =
              λhead_is_expected.
                different
                  (trans
                    Int
                    actual
                    (uint8_to_int head)
                    expected
                    (sym Int (uint8_to_int head) actual head_is_actual)
                    head_is_expected)
          in
            byte_code_rejected_from_head
              cur
              expected
              head
              (list_append UInt8 tail rest)
              starts_with
              head_is_different
  }

theorem bytes_concat_three_view
      (a : Bytes) (b : Bytes) (c : Bytes)
    : Equal
        (List UInt8)
        (bytes_to_list (bytes_concat (bytes_concat a b) c))
        (list_append
          UInt8
          (bytes_to_list a)
          (list_append UInt8 (bytes_to_list b) (bytes_to_list c))) =
  trans
    (List UInt8)
    (bytes_to_list (bytes_concat (bytes_concat a b) c))
    (list_append
      UInt8
      (list_append UInt8 (bytes_to_list a) (bytes_to_list b))
      (bytes_to_list c))
    (list_append
      UInt8
      (bytes_to_list a)
      (list_append UInt8 (bytes_to_list b) (bytes_to_list c)))
    (trans
      (List UInt8)
      (bytes_to_list (bytes_concat (bytes_concat a b) c))
      (list_append UInt8 (bytes_to_list (bytes_concat a b)) (bytes_to_list c))
      (list_append
        UInt8
        (list_append UInt8 (bytes_to_list a) (bytes_to_list b))
        (bytes_to_list c))
      (bytes_concat_list_view (bytes_concat a b) c)
      (cong
        (List UInt8)
        (List UInt8)
        (bytes_to_list (bytes_concat a b))
        (list_append UInt8 (bytes_to_list a) (bytes_to_list b))
        (λprefix. list_append UInt8 prefix (bytes_to_list c))
        (bytes_concat_list_view a b)))
    ((proof assoc for list_append) UInt8 (bytes_to_list a) (bytes_to_list b) (bytes_to_list c))

theorem bytes_concat_five_view
      (a : Bytes) (b : Bytes) (c : Bytes) (d : Bytes) (e : Bytes)
    : Equal
        (List UInt8)
        (bytes_to_list (bytes_concat (bytes_concat (bytes_concat a b) c) (bytes_concat d e)))
        (list_append
          UInt8
          (bytes_to_list a)
          (list_append
            UInt8
            (bytes_to_list b)
            (list_append
              UInt8
              (bytes_to_list c)
              (list_append UInt8 (bytes_to_list d) (bytes_to_list e))))) =
  let
    a_codes : List UInt8 = bytes_to_list a;
    b_codes : List UInt8 = bytes_to_list b;
    c_codes : List UInt8 = bytes_to_list c;
    d_codes : List UInt8 = bytes_to_list d;
    e_codes : List UInt8 = bytes_to_list e;
    first_three : Bytes = bytes_concat (bytes_concat a b) c;
    last_two : Bytes = bytes_concat d e;
    prefix : List UInt8 = list_append UInt8 a_codes (list_append UInt8 b_codes c_codes);
    suffix : List UInt8 = list_append UInt8 d_codes e_codes;
    first_view : Equal (List UInt8) (bytes_to_list first_three) prefix =
      bytes_concat_three_view a b c;
    last_view : Equal (List UInt8) (bytes_to_list last_two) suffix = bytes_concat_list_view d e;
    prefix_extended : Equal
      (List UInt8)
      (list_append UInt8 (bytes_to_list first_three) (bytes_to_list last_two))
      (list_append UInt8 prefix (bytes_to_list last_two)) =
      cong
        (List UInt8)
        (List UInt8)
        (bytes_to_list first_three)
        prefix
        (λcodes. list_append UInt8 codes (bytes_to_list last_two))
        first_view;
    suffix_extended : Equal
      (List UInt8)
      (list_append UInt8 prefix (bytes_to_list last_two))
      (list_append UInt8 prefix suffix) =
      cong
        (List UInt8)
        (List UInt8)
        (bytes_to_list last_two)
        suffix
        (λcodes. list_append UInt8 prefix codes)
        last_view;
    associate_suffix : Equal
      (List UInt8)
      (list_append UInt8 prefix suffix)
      (list_append
        UInt8
        a_codes
        (list_append UInt8 b_codes (list_append UInt8 c_codes suffix))) =
      trans
        (List UInt8)
        (list_append UInt8 prefix suffix)
        (list_append
          UInt8
          a_codes
          (list_append UInt8 (list_append UInt8 b_codes c_codes) suffix))
        (list_append
          UInt8
          a_codes
          (list_append UInt8 b_codes (list_append UInt8 c_codes suffix)))
        ((proof assoc for list_append) UInt8 a_codes (list_append UInt8 b_codes c_codes) suffix)
        (cong
          (List UInt8)
          (List UInt8)
          (list_append UInt8 (list_append UInt8 b_codes c_codes) suffix)
          (list_append UInt8 b_codes (list_append UInt8 c_codes suffix))
          (λcodes. list_append UInt8 a_codes codes)
          ((proof assoc for list_append) UInt8 b_codes c_codes suffix))
  in
    trans
      (List UInt8)
      (bytes_to_list (bytes_concat first_three last_two))
      (list_append UInt8 (bytes_to_list first_three) (bytes_to_list last_two))
      (list_append UInt8 a_codes (list_append UInt8 b_codes (list_append UInt8 c_codes suffix)))
      (bytes_concat_list_view first_three last_two)
      (trans
        (List UInt8)
        (list_append UInt8 (bytes_to_list first_three) (bytes_to_list last_two))
        (list_append UInt8 prefix (bytes_to_list last_two))
        (list_append
          UInt8
          a_codes
          (list_append UInt8 b_codes (list_append UInt8 c_codes suffix)))
        prefix_extended
        (trans
          (List UInt8)
          (list_append UInt8 prefix (bytes_to_list last_two))
          (list_append UInt8 prefix suffix)
          (list_append
            UInt8
            a_codes
            (list_append UInt8 b_codes (list_append UInt8 c_codes suffix)))
          suffix_extended
          associate_suffix))

theorem list_append_five_suffix
      (a : Type)
      (first : List a)
      (second : List a)
      (third : List a)
      (fourth : List a)
      (fifth : List a)
      (rest : List a)
    : Equal
        (List a)
        (list_append
          a
          (list_append
            a
            first
            (list_append a second (list_append a third (list_append a fourth fifth))))
          rest)
        (list_append
          a
          first
          (list_append
            a
            second
            (list_append a third (list_append a fourth (list_append a fifth rest))))) =
  let
    tail_one : List a = list_append a second (list_append a third (list_append a fourth fifth));
    tail_two : List a = list_append a third (list_append a fourth fifth);
    tail_three : List a = list_append a fourth fifth;
    start : List a = list_append a (list_append a first tail_one) rest;
    after_first : List a = list_append a first (list_append a tail_one rest);
    after_second : List a =
      list_append a first (list_append a second (list_append a tail_two rest));
    after_third : List a =
      list_append
        a
        first
        (list_append a second (list_append a third (list_append a tail_three rest)));
    after_fourth : List a =
      list_append
        a
        first
        (list_append
          a
          second
          (list_append a third (list_append a fourth (list_append a fifth rest))))
  in
    trans
      (List a)
      start
      after_first
      after_fourth
      ((proof assoc for list_append) a first tail_one rest)
      (trans
        (List a)
        after_first
        after_second
        after_fourth
        (cong
          (List a)
          (List a)
          (list_append a tail_one rest)
          (list_append a second (list_append a tail_two rest))
          (λxs. list_append a first xs)
          ((proof assoc for list_append) a second tail_two rest))
        (trans
          (List a)
          after_second
          after_third
          after_fourth
          (cong
            (List a)
            (List a)
            (list_append a tail_two rest)
            (list_append a third (list_append a tail_three rest))
            (λxs. list_append a first (list_append a second xs))
            ((proof assoc for list_append) a third tail_three rest))
          (cong
            (List a)
            (List a)
            (list_append a tail_three rest)
            (list_append a fourth (list_append a fifth rest))
            (λxs. list_append a first (list_append a second (list_append a third xs)))
            ((proof assoc for list_append) a fourth fifth rest))))

theorem printed_not_bytes_view
      (child : BoolExpr)
    : Equal
        (List UInt8)
        (bytes_to_list (print_bool_expr (BNot child)))
        (list_append
          UInt8
          (bytes_to_list (bytes_encode not_open_text))
          (list_append
            UInt8
            (bytes_to_list (print_bool_expr child))
            (bytes_to_list (bytes_encode close_text)))) =
  bytes_concat_three_view
    (bytes_encode not_open_text)
    (print_bool_expr child)
    (bytes_encode close_text)

theorem printed_and_bytes_view
      (left : BoolExpr) (right : BoolExpr)
    : Equal
        (List UInt8)
        (bytes_to_list (print_bool_expr (BAnd left right)))
        (list_append
          UInt8
          (bytes_to_list (bytes_encode and_open_text))
          (list_append
            UInt8
            (bytes_to_list (print_bool_expr left))
            (list_append
              UInt8
              (bytes_to_list (bytes_encode separator_text))
              (list_append
                UInt8
                (bytes_to_list (print_bool_expr right))
                (bytes_to_list (bytes_encode close_text)))))) =
  bytes_concat_five_view
    (bytes_encode and_open_text)
    (print_bool_expr left)
    (bytes_encode separator_text)
    (print_bool_expr right)
    (bytes_encode close_text)

theorem printed_not_suffix_view
      (child : BoolExpr) (rest : List UInt8)
    : Equal
        (List UInt8)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest)
        (list_append
          UInt8
          (bytes_to_list (bytes_encode not_open_text))
          (list_append
            UInt8
            (bytes_to_list (print_bool_expr child))
            (list_append UInt8 (bytes_to_list (bytes_encode close_text)) rest))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode not_open_text);
    child_bytes : List UInt8 = bytes_to_list (print_bool_expr child);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    unfolded : List UInt8 =
      list_append UInt8 open_bytes (list_append UInt8 child_bytes close_bytes);
    after_open : List UInt8 =
      list_append
        UInt8
        open_bytes
        (list_append UInt8 (list_append UInt8 child_bytes close_bytes) rest)
  in
    trans
      (List UInt8)
      (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest)
      after_open
      (list_append
        UInt8
        open_bytes
        (list_append UInt8 child_bytes (list_append UInt8 close_bytes rest)))
      (trans
        (List UInt8)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest)
        (list_append UInt8 unfolded rest)
        after_open
        (cong
          (List UInt8)
          (List UInt8)
          (bytes_to_list (print_bool_expr (BNot child)))
          unfolded
          (λprefix. list_append UInt8 prefix rest)
          (printed_not_bytes_view child))
        ((proof assoc for list_append)
          UInt8
          open_bytes
          (list_append UInt8 child_bytes close_bytes)
          rest))
      (cong
        (List UInt8)
        (List UInt8)
        (list_append UInt8 (list_append UInt8 child_bytes close_bytes) rest)
        (list_append UInt8 child_bytes (list_append UInt8 close_bytes rest))
        (λsuffix. list_append UInt8 open_bytes suffix)
        ((proof assoc for list_append) UInt8 child_bytes close_bytes rest))

theorem printed_and_suffix_view
      (left : BoolExpr) (right : BoolExpr) (rest : List UInt8)
    : Equal
        (List UInt8)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest)
        (list_append
          UInt8
          (bytes_to_list (bytes_encode and_open_text))
          (list_append
            UInt8
            (bytes_to_list (print_bool_expr left))
            (list_append
              UInt8
              (bytes_to_list (bytes_encode separator_text))
              (list_append
                UInt8
                (bytes_to_list (print_bool_expr right))
                (list_append UInt8 (bytes_to_list (bytes_encode close_text)) rest))))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode and_open_text);
    left_bytes : List UInt8 = bytes_to_list (print_bool_expr left);
    separator_bytes : List UInt8 = bytes_to_list (bytes_encode separator_text);
    right_bytes : List UInt8 = bytes_to_list (print_bool_expr right);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    unfolded : List UInt8 =
      list_append
        UInt8
        open_bytes
        (list_append
          UInt8
          left_bytes
          (list_append UInt8 separator_bytes (list_append UInt8 right_bytes close_bytes)))
  in
    trans
      (List UInt8)
      (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest)
      (list_append UInt8 unfolded rest)
      (list_append
        UInt8
        open_bytes
        (list_append
          UInt8
          left_bytes
          (list_append
            UInt8
            separator_bytes
            (list_append UInt8 right_bytes (list_append UInt8 close_bytes rest)))))
      (cong
        (List UInt8)
        (List UInt8)
        (bytes_to_list (print_bool_expr (BAnd left right)))
        unfolded
        (λprefix. list_append UInt8 prefix rest)
        (printed_and_bytes_view left right))
      (list_append_five_suffix
        UInt8
        open_bytes
        left_bytes
        separator_bytes
        right_bytes
        close_bytes
        rest)

theorem spaces_on_nonspace_prefix
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (actual : Int)
      (actual_rest : List Int)
      (different : Equal Int actual separator_code → Bottom)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int actual actual_rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  decoder_many_rejected_succeeds
    ByteCursor
    UInt8
    Span
    UInt8
    byte_cursor_ops
    (byte_code_decoder separator_code)
    cur
    (byte_cursor_locate cur)
    (byte_code_rejects_different_prefix
      cur
      separator_code
      actual
      actual_rest
      prefix
      rest
      different
      starts_with
      mapped)

theorem spaces_on_ascii_token
      (text : String)
      (ascii : AllAscii text)
      (actual : Int)
      (actual_rest : List Int)
      (codes : Equal
        (List Int)
        (map Char Int charToInt (string_to_list_char text))
        (Cons Int actual actual_rest))
      (different : Equal Int actual separator_code → Bottom)
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  spaces_on_nonspace_prefix
    cur
    (bytes_to_list (bytes_encode text))
    rest
    actual
    actual_rest
    different
    starts_with
    (trans
      (List Int)
      (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode text)))
      (map Char Int charToInt (string_to_list_char text))
      (Cons Int actual actual_rest)
      (bytes_encode_ascii_octets text ascii)
      codes)

theorem spaces_on_true_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode true_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  spaces_on_ascii_token
    true_token_text
    true_token_ascii
    true_initial_code
    true_remaining_codes
    Refl
    (λsame. absurd same)
    cur
    rest
    starts_with

theorem spaces_on_false_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode false_token_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  spaces_on_ascii_token
    false_token_text
    false_token_ascii
    false_initial_code
    false_remaining_codes
    Refl
    (λsame. absurd same)
    cur
    rest
    starts_with

theorem spaces_on_not_open_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode not_open_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  spaces_on_ascii_token
    not_open_text
    not_open_ascii
    open_initial_code
    (Cons Int 110 (Cons Int 111 (Cons Int 116 (Cons Int 32 (Nil Int)))))
    Refl
    (λsame. absurd same)
    cur
    rest
    starts_with

theorem spaces_on_and_open_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode and_open_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  spaces_on_ascii_token
    and_open_text
    and_open_ascii
    open_initial_code
    (Cons Int 97 (Cons Int 110 (Cons Int 100 (Cons Int 32 (Nil Int)))))
    Refl
    (λsame. absurd same)
    cur
    rest
    starts_with

theorem spaces_on_close_token
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (bytes_encode close_text)) rest))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  spaces_on_ascii_token
    close_text
    close_ascii
    close_code
    (Nil Int)
    Refl
    (λsame. absurd same)
    cur
    rest
    starts_with

theorem spaces_on_empty_suffix
      (cur : ByteCursor) (empty : Equal (List UInt8) (source_suffix cur) (Nil UInt8))
    : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  let
    peeked_none : Equal (Option UInt8) (byte_cursor_peek cur) (None UInt8) =
      trans
        (Option UInt8)
        (byte_cursor_peek cur)
        (nth UInt8 Zero (source_suffix cur))
        (None UInt8)
        (list_drop_at_nth
          UInt8
          (byte_cursor_position cur)
          (bytes_to_list (source_bytes (byte_cursor_source cur))))
        (cong
          (List UInt8)
          (Option UInt8)
          (source_suffix cur)
          (Nil UInt8)
          (nth UInt8 Zero)
          empty);
    step_rejected : Equal
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_decoder separator_code cur)
      (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur))) =
      trans
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder separator_code cur)
        (byte_code_result_peek cur separator_code (byte_cursor_peek cur))
        (DecoderFailed ByteCursor Span UInt8 (DecoderRejected Span (byte_cursor_locate cur)))
        (byte_code_result_peek_equation cur separator_code)
        (cong
          (Option UInt8)
          (DecoderResult ByteCursor Span UInt8)
          (byte_cursor_peek cur)
          (None UInt8)
          (λobserved. byte_code_result_peek cur separator_code observed)
          peeked_none)
  in
    decoder_many_rejected_succeeds
      ByteCursor
      UInt8
      Span
      UInt8
      byte_cursor_ops
      (byte_code_decoder separator_code)
      cur
      (byte_cursor_locate cur)
      step_rejected

theorem spaces_on_printed_expr
      (e : BoolExpr)
    : (cur : ByteCursor)
      → (rest : List UInt8)
      → Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr e)) rest)
      → Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder cur)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
  match e {
    BTrue ↦ λcur. λrest. λstarts_with. spaces_on_true_token cur rest starts_with;
    BFalse ↦ λcur. λrest. λstarts_with. spaces_on_false_token cur rest starts_with;
    BNot child ↦
      λcur.
        λrest.
          λstarts_with.
            let
              child_and_close : List UInt8 =
                list_append
                  UInt8
                  (bytes_to_list (print_bool_expr child))
                  (list_append UInt8 (bytes_to_list (bytes_encode close_text)) rest);
              open_starts : Equal
                (List UInt8)
                (source_suffix cur)
                (list_append
                  UInt8
                  (bytes_to_list (bytes_encode not_open_text))
                  child_and_close) =
                trans
                  (List UInt8)
                  (source_suffix cur)
                  (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest)
                  (list_append
                    UInt8
                    (bytes_to_list (bytes_encode not_open_text))
                    child_and_close)
                  starts_with
                  (printed_not_suffix_view child rest)
            in
              spaces_on_not_open_token cur child_and_close open_starts;
    BAnd left right ↦
      λcur.
        λrest.
          λstarts_with.
            let
              after_open : List UInt8 =
                list_append
                  UInt8
                  (bytes_to_list (print_bool_expr left))
                  (list_append
                    UInt8
                    (bytes_to_list (bytes_encode separator_text))
                    (list_append
                      UInt8
                      (bytes_to_list (print_bool_expr right))
                      (list_append UInt8 (bytes_to_list (bytes_encode close_text)) rest)));
              open_starts : Equal
                (List UInt8)
                (source_suffix cur)
                (list_append UInt8 (bytes_to_list (bytes_encode and_open_text)) after_open) =
                trans
                  (List UInt8)
                  (source_suffix cur)
                  (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest)
                  (list_append UInt8 (bytes_to_list (bytes_encode and_open_text)) after_open)
                  starts_with
                  (printed_and_suffix_view left right rest)
            in
              spaces_on_and_open_token cur after_open open_starts
  }

fn printed_end (cur : ByteCursor) (e : BoolExpr) : ByteCursor =
  cursor_after_codes cur (bytes_to_list (print_bool_expr e))

fn printed_syntax (cur : ByteCursor) (e : BoolExpr) : Syntax BoolExpr =
  match e {
    BTrue ↦
      syntax_leaf
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position (printed_end cur BTrue))
        BTrue;
    BFalse ↦
      syntax_leaf
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position (printed_end cur BFalse))
        BFalse;
    BNot child ↦
      let
        after_open : ByteCursor =
          cursor_after_codes cur (bytes_to_list (bytes_encode not_open_text));
        child_syntax : Syntax BoolExpr = printed_syntax after_open child
      in
        syntax_node_unary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position (printed_end cur (BNot child)))
          (BNot child)
          child_syntax;
    BAnd left right ↦
      let
        after_open : ByteCursor =
          cursor_after_codes cur (bytes_to_list (bytes_encode and_open_text));
        after_left : ByteCursor = printed_end after_open left;
        after_separator : ByteCursor =
          cursor_after_codes after_left (bytes_to_list (bytes_encode separator_text));
        left_syntax : Syntax BoolExpr = printed_syntax after_open left;
        right_syntax : Syntax BoolExpr = printed_syntax after_separator right
      in
        syntax_node_binary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position (printed_end cur (BAnd left right)))
          (BAnd left right)
          left_syntax
          right_syntax
  }

theorem printed_syntax_erases
      (e : BoolExpr)
    : (cur : ByteCursor) → Equal BoolExpr (erase_spans (printed_syntax cur e)) e =
  match e {
    BTrue ↦ λcur. Proved;
    BFalse ↦ λcur. Proved;
    BNot child ↦ λcur. Refl;
    BAnd left right ↦ λcur. Refl
  }

theorem cursor_after_codes_respects_view
      (cur : ByteCursor)
      (first : List UInt8)
      (second : List UInt8)
      (same_codes : Equal (List UInt8) first second)
    : Equal ByteCursor (cursor_after_codes cur first) (cursor_after_codes cur second) =
  cong (List UInt8) ByteCursor first second (λcodes. cursor_after_codes cur codes) same_codes

theorem printed_not_end_view
      (cur : ByteCursor) (child : BoolExpr)
    : Equal ByteCursor
        (printed_end cur (BNot child))
        (cursor_after_codes
          (printed_end
            (cursor_after_codes cur (bytes_to_list (bytes_encode not_open_text)))
            child)
          (bytes_to_list (bytes_encode close_text))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode not_open_text);
    child_bytes : List UInt8 = bytes_to_list (print_bool_expr child);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    after_open : ByteCursor = cursor_after_codes cur open_bytes;
    child_end : ByteCursor = printed_end after_open child;
    after_unfolded : ByteCursor =
      cursor_after_codes
        cur
        (list_append UInt8 open_bytes (list_append UInt8 child_bytes close_bytes));
    after_prefix : ByteCursor =
      cursor_after_codes after_open (list_append UInt8 child_bytes close_bytes)
  in
    trans
      ByteCursor
      (printed_end cur (BNot child))
      after_unfolded
      (cursor_after_codes child_end close_bytes)
      (cursor_after_codes_respects_view
        cur
        (bytes_to_list (print_bool_expr (BNot child)))
        (list_append UInt8 open_bytes (list_append UInt8 child_bytes close_bytes))
        (printed_not_bytes_view child))
      (trans
        ByteCursor
        after_unfolded
        after_prefix
        (cursor_after_codes child_end close_bytes)
        (cursor_after_codes_append cur open_bytes (list_append UInt8 child_bytes close_bytes))
        (cursor_after_codes_append after_open child_bytes close_bytes))

theorem printed_and_end_view
      (cur : ByteCursor) (left : BoolExpr) (right : BoolExpr)
    : Equal ByteCursor
        (printed_end cur (BAnd left right))
        (cursor_after_codes
          (printed_end
            (cursor_after_codes
              (printed_end
                (cursor_after_codes cur (bytes_to_list (bytes_encode and_open_text)))
                left)
              (bytes_to_list (bytes_encode separator_text)))
            right)
          (bytes_to_list (bytes_encode close_text))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode and_open_text);
    left_bytes : List UInt8 = bytes_to_list (print_bool_expr left);
    separator_bytes : List UInt8 = bytes_to_list (bytes_encode separator_text);
    right_bytes : List UInt8 = bytes_to_list (print_bool_expr right);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    after_open : ByteCursor = cursor_after_codes cur open_bytes;
    after_left : ByteCursor = printed_end after_open left;
    after_separator : ByteCursor = cursor_after_codes after_left separator_bytes;
    after_right : ByteCursor = printed_end after_separator right;
    rest_after_right : List UInt8 = list_append UInt8 right_bytes close_bytes;
    rest_after_separator : List UInt8 = list_append UInt8 separator_bytes rest_after_right;
    rest_after_left : List UInt8 = list_append UInt8 left_bytes rest_after_separator;
    unfolded : ByteCursor =
      cursor_after_codes cur (list_append UInt8 open_bytes rest_after_left);
    after_open_view : ByteCursor = cursor_after_codes after_open rest_after_left;
    after_left_view : ByteCursor = cursor_after_codes after_left rest_after_separator;
    after_separator_view : ByteCursor = cursor_after_codes after_separator rest_after_right;
    after_close : ByteCursor = cursor_after_codes after_right close_bytes
  in
    trans
      ByteCursor
      (printed_end cur (BAnd left right))
      unfolded
      after_close
      (cursor_after_codes_respects_view
        cur
        (bytes_to_list (print_bool_expr (BAnd left right)))
        (list_append UInt8 open_bytes rest_after_left)
        (printed_and_bytes_view left right))
      (trans
        ByteCursor
        unfolded
        after_open_view
        after_close
        (cursor_after_codes_append cur open_bytes rest_after_left)
        (trans
          ByteCursor
          after_open_view
          after_left_view
          after_close
          (cursor_after_codes_append after_open left_bytes rest_after_separator)
          (trans
            ByteCursor
            after_left_view
            after_separator_view
            after_close
            (cursor_after_codes_append after_left separator_bytes rest_after_right)
            (cursor_after_codes_append after_separator right_bytes close_bytes))))

fn PrintedBoolSpecAt
      (expr : BoolExpr) (cur : ByteCursor) (syntax : Syntax BoolExpr) (next : ByteCursor)
    : Prop =
  And
    (Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 (bytes_to_list (print_bool_expr expr)) (source_suffix next)))
    (And
      (Equal (Syntax BoolExpr) syntax (printed_syntax cur expr))
      (Equal ByteCursor next (printed_end cur expr)))

fn PrintedBoolSpec (cur : ByteCursor) (syntax : Syntax BoolExpr) (next : ByteCursor) : Prop =
  PrintedBoolSpecAt (erase_spans syntax) cur syntax next

theorem cursor_after_nonempty_strict
      (codes : List UInt8)
    : (cur : ByteCursor)
      → (rest : List UInt8)
      → Equal (List UInt8) (source_suffix cur) (list_append UInt8 codes rest)
      → ListNonempty UInt8 codes
      → Equal Bool
        (cursor_nat_lt
          (byte_cursor_remaining (cursor_after_codes cur codes))
          (byte_cursor_remaining cur))
        True =
  match codes {
    Nil ↦ λcur. λrest. λstarts_with. λnonempty. absurd nonempty;
    Cons first more ↦
      λcur.
        λrest.
          λstarts_with.
            λnonempty. cursor_after_codes_nonempty_strict cur first more rest starts_with
  }

theorem printed_end_self
      (e : BoolExpr) (cur : ByteCursor)
    : Equal ByteCursor (printed_end cur e) (printed_end cur e) =
  cong
    Nat
    ByteCursor
    (list_advance_over UInt8 (byte_cursor_position cur) (bytes_to_list (print_bool_expr e)))
    (list_advance_over UInt8 (byte_cursor_position cur) (bytes_to_list (print_bool_expr e)))
    (λposition. MkByteCursor (byte_cursor_source cur) position)
    Refl

theorem printed_bool_spec_intro
      (e : BoolExpr)
      (cur : ByteCursor)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr e)) rest))
    : PrintedBoolSpec cur (printed_syntax cur e) (printed_end cur e) =
  let
    syntax : Syntax BoolExpr = printed_syntax cur e;
    next : ByteCursor = printed_end cur e;
    printed_bytes : List UInt8 = bytes_to_list (print_bool_expr e);
    rest_after_print : Equal (List UInt8) (source_suffix next) rest =
      source_suffix_after_codes cur printed_bytes rest starts_with;
    prefix_view : Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 printed_bytes (source_suffix next)) =
      trans
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 printed_bytes rest)
        (list_append UInt8 printed_bytes (source_suffix next))
        starts_with
        (cong
          (List UInt8)
          (List UInt8)
          rest
          (source_suffix next)
          (λtail. list_append UInt8 printed_bytes tail)
          (sym (List UInt8) (source_suffix next) rest rest_after_print));
    at_expr : PrintedBoolSpecAt e cur syntax next =
      and_intro
        (Equal
          (List UInt8)
          (source_suffix cur)
          (list_append UInt8 printed_bytes (source_suffix next)))
        (And
          (Equal (Syntax BoolExpr) syntax (printed_syntax cur e))
          (Equal ByteCursor next (printed_end cur e)))
        prefix_view
        (and_intro
          (Equal (Syntax BoolExpr) (printed_syntax cur e) (printed_syntax cur e))
          (Equal ByteCursor (printed_end cur e) (printed_end cur e))
          Refl
          (printed_end_self e cur))
  in
    J
      (λexpr _. PrintedBoolSpecAt expr cur syntax next)
      at_expr
      (sym BoolExpr (erase_spans syntax) e (printed_syntax_erases e cur))

theorem printed_bool_spec_prefix
      (e : BoolExpr)
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpecAt e cur syntax next)
    : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr e)) (source_suffix next)) =
  and_fst
    (Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 (bytes_to_list (print_bool_expr e)) (source_suffix next)))
    (And
      (Equal (Syntax BoolExpr) syntax (printed_syntax cur e))
      (Equal ByteCursor next (printed_end cur e)))
    holds

theorem printed_bool_spec_syntax
      (e : BoolExpr)
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpecAt e cur syntax next)
    : Equal (Syntax BoolExpr) syntax (printed_syntax cur e) =
  and_fst
    (Equal (Syntax BoolExpr) syntax (printed_syntax cur e))
    (Equal ByteCursor next (printed_end cur e))
    (and_snd
      (Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr e)) (source_suffix next)))
      (And
        (Equal (Syntax BoolExpr) syntax (printed_syntax cur e))
        (Equal ByteCursor next (printed_end cur e)))
      holds)

theorem printed_bool_spec_end
      (e : BoolExpr)
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpecAt e cur syntax next)
    : Equal ByteCursor next (printed_end cur e) =
  and_snd
    (Equal (Syntax BoolExpr) syntax (printed_syntax cur e))
    (Equal ByteCursor next (printed_end cur e))
    (and_snd
      (Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr e)) (source_suffix next)))
      (And
        (Equal (Syntax BoolExpr) syntax (printed_syntax cur e))
        (Equal ByteCursor next (printed_end cur e)))
      holds)

theorem decoded_bool_result_transport
      (first_syntax : Syntax BoolExpr)
      (second_syntax : Syntax BoolExpr)
      (first_next : ByteCursor)
      (second_next : ByteCursor)
      (same_syntax : Equal (Syntax BoolExpr) first_syntax second_syntax)
      (same_next : Equal ByteCursor first_next second_next)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (Decoded ByteCursor Span (Syntax BoolExpr) first_syntax first_next)
        (Decoded ByteCursor Span (Syntax BoolExpr) second_syntax second_next) =
  trans
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (Decoded ByteCursor Span (Syntax BoolExpr) first_syntax first_next)
    (Decoded ByteCursor Span (Syntax BoolExpr) second_syntax first_next)
    (Decoded ByteCursor Span (Syntax BoolExpr) second_syntax second_next)
    (cong
      (Syntax BoolExpr)
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      first_syntax
      second_syntax
      (λsyntax. Decoded ByteCursor Span (Syntax BoolExpr) syntax first_next)
      same_syntax)
    (cong
      ByteCursor
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      first_next
      second_next
      (λnext. Decoded ByteCursor Span (Syntax BoolExpr) second_syntax next)
      same_next)

theorem bool_layer_from_canonical
      (e : BoolExpr)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpecAt e cur syntax next)
      (canonical_succeeds : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) (printed_syntax cur e) (printed_end cur e)))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
  let
    expected_syntax : Syntax BoolExpr = printed_syntax cur e;
    expected_next : ByteCursor = printed_end cur e;
    syntax_matches : Equal (Syntax BoolExpr) expected_syntax syntax =
      sym
        (Syntax BoolExpr)
        syntax
        expected_syntax
        (printed_bool_spec_syntax e cur syntax next holds);
    next_matches : Equal ByteCursor expected_next next =
      sym ByteCursor next expected_next (printed_bool_spec_end e cur syntax next holds)
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_decoder_layer recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) expected_syntax expected_next)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
      canonical_succeeds
      (decoded_bool_result_transport
        expected_syntax
        syntax
        expected_next
        next
        syntax_matches
        next_matches)

theorem bool_layer_true_for_printed_spec
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpecAt BTrue cur syntax next)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
  bool_layer_from_canonical
    BTrue
    recur
    cur
    syntax
    next
    holds
    (bool_layer_true_on_printed_token
      recur
      cur
      (source_suffix next)
      (printed_bool_spec_prefix BTrue cur syntax next holds))

theorem bool_layer_false_for_printed_spec
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpecAt BFalse cur syntax next)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
  bool_layer_from_canonical
    BFalse
    recur
    cur
    syntax
    next
    holds
    (bool_layer_false_on_printed_token
      recur
      cur
      (source_suffix next)
      (printed_bool_spec_prefix BFalse cur syntax next holds))

theorem decoder_then_success
      (a : Type)
      (b : Type)
      (outcome : DecoderResult ByteCursor Span a)
      (value : a)
      (next : ByteCursor)
      (continue : a → ByteCursor → DecoderResult ByteCursor Span b)
      (succeeds : Equal
        (DecoderResult ByteCursor Span a)
        outcome
        (Decoded ByteCursor Span a value next))
    : Equal
        (DecoderResult ByteCursor Span b)
        (decoder_then_result a b outcome continue)
        (continue value next) =
  cong
    (DecoderResult ByteCursor Span a)
    (DecoderResult ByteCursor Span b)
    outcome
    (Decoded ByteCursor Span a value next)
    (λobserved. decoder_then_result a b observed continue)
    succeeds

theorem bool_layer_open_success
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (more_codes : List Int)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int open_initial_code more_codes))
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (last_succeeds : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_layer_last recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
  let
    reject_true : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_true_decoder cur)
      (DecoderFailed
        ByteCursor
        Span
        (Syntax BoolExpr)
        (DecoderRejected Span (byte_cursor_locate cur))) =
      bool_true_rejected_by_initial
        cur
        prefix
        rest
        open_initial_code
        more_codes
        (λsame. absurd same)
        starts_with
        mapped;
    reject_false : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_false_decoder cur)
      (DecoderFailed
        ByteCursor
        Span
        (Syntax BoolExpr)
        (DecoderRejected Span (byte_cursor_locate cur))) =
      bool_false_rejected_by_initial
        cur
        prefix
        rest
        open_initial_code
        more_codes
        (λsame. absurd same)
        starts_with
        mapped;
    true_uses_rest : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_decoder_layer recur cur)
      (bool_layer_rest recur cur) =
      decoder_alt_rejection_uses_second
        ByteCursor
        Span
        (Syntax BoolExpr)
        bool_true_decoder
        (bool_layer_rest recur)
        cur
        (byte_cursor_locate cur)
        reject_true;
    false_uses_last : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_layer_rest recur cur)
      (bool_layer_last recur cur) =
      decoder_alt_rejection_uses_second
        ByteCursor
        Span
        (Syntax BoolExpr)
        bool_false_decoder
        (bool_layer_last recur)
        cur
        (byte_cursor_locate cur)
        reject_false
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_decoder_layer recur cur)
      (bool_layer_rest recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
      true_uses_rest
      (trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_layer_rest recur cur)
        (bool_layer_last recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
        false_uses_last
        last_succeeds)

fn bool_not_continue
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (ignored : UInt8)
      (after_open : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  not_spaces_after cur recur after_open

theorem bool_not_result_pointwise
      (recur : Decoder ByteCursor Span (Syntax BoolExpr)) (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_not_decoder recur cur)
        (decoder_then_result
          UInt8
          (Syntax BoolExpr)
          (not_open_token_decoder cur)
          (bool_not_continue cur recur)) =
  Refl

theorem bool_not_succeeds_on_parts
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (after_open : ByteCursor)
      (child_syntax : Syntax BoolExpr)
      (child_end : ByteCursor)
      (after_close : ByteCursor)
      (open_byte : UInt8)
      (close_byte : UInt8)
      (open_succeeds : Equal
        (DecoderResult ByteCursor Span UInt8)
        (not_open_token_decoder cur)
        (Decoded ByteCursor Span UInt8 open_byte after_open))
      (opening_spaces : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder after_open)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) after_open))
      (child_succeeds : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur after_open)
        (Decoded ByteCursor Span (Syntax BoolExpr) child_syntax child_end))
      (closing_spaces : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder child_end)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) child_end))
      (close_succeeds : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder close_code child_end)
        (Decoded ByteCursor Span UInt8 close_byte after_close))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_not_decoder recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (syntax_node_unary
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position after_close)
            (BNot (erase_spans child_syntax))
            child_syntax)
          after_close) =
  let
    closed : Syntax BoolExpr =
      syntax_node_unary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BNot (erase_spans child_syntax))
        child_syntax;
    close_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (not_close_after cur child_syntax child_end)
      (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close) =
      decoder_then_success
        UInt8
        (Syntax BoolExpr)
        (byte_code_decoder close_code child_end)
        close_byte
        after_close
        (λignored.
          λend.
            Decoded
              ByteCursor
              Span
              (Syntax BoolExpr)
              (syntax_node_unary
                (byte_cursor_source cur)
                (byte_cursor_position cur)
                (byte_cursor_position end)
                (BNot (erase_spans child_syntax))
                child_syntax)
              end)
        close_succeeds;
    trailing_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (not_trailing_after cur child_syntax child_end)
      (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close) =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (not_trailing_after cur child_syntax child_end)
        (not_close_after cur child_syntax child_end)
        (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close)
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder child_end)
          (Nil UInt8)
          child_end
          (λignored. λclose_start. not_close_after cur child_syntax close_start)
          closing_spaces)
        close_stage;
    child_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (not_child_after cur recur after_open)
      (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close) =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (not_child_after cur recur after_open)
        (not_trailing_after cur child_syntax child_end)
        (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close)
        (decoder_then_success
          (Syntax BoolExpr)
          (Syntax BoolExpr)
          (recur after_open)
          child_syntax
          child_end
          (λchild. λend. not_trailing_after cur child end)
          child_succeeds)
        trailing_stage;
    spaces_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (not_spaces_after cur recur after_open)
      (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close) =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (not_spaces_after cur recur after_open)
        (not_child_after cur recur after_open)
        (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close)
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder after_open)
          (Nil UInt8)
          after_open
          (λignored. λchild_start. not_child_after cur recur child_start)
          opening_spaces)
        child_stage
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_not_decoder recur cur)
      (not_spaces_after cur recur after_open)
      (Decoded ByteCursor Span (Syntax BoolExpr) closed after_close)
      (trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_not_decoder recur cur)
        (decoder_then_result
          UInt8
          (Syntax BoolExpr)
          (not_open_token_decoder cur)
          (bool_not_continue cur recur))
        (not_spaces_after cur recur after_open)
        (bool_not_result_pointwise recur cur)
        (decoder_then_success
          UInt8
          (Syntax BoolExpr)
          (not_open_token_decoder cur)
          open_byte
          after_open
          (bool_not_continue cur recur)
          open_succeeds))
      spaces_stage

theorem bool_not_decoder_succeeds_printed
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (child : BoolExpr)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest))
      (recur_works : (inner : ByteCursor)
        → (syntax : Syntax BoolExpr)
        → (next : ByteCursor)
        → Equal
        Bool
        (cursor_nat_lt (byte_cursor_remaining inner) (byte_cursor_remaining cur))
        True
        → PrintedBoolSpec
        inner
        syntax
        next
        → Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur inner)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_not_decoder recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (printed_syntax cur (BNot child))
          (printed_end cur (BNot child))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode not_open_text);
    child_bytes : List UInt8 = bytes_to_list (print_bool_expr child);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    after_open : ByteCursor = cursor_after_codes cur open_bytes;
    child_syntax : Syntax BoolExpr = printed_syntax after_open child;
    child_end : ByteCursor = printed_end after_open child;
    after_close : ByteCursor = cursor_after_codes child_end close_bytes;
    child_rest : List UInt8 = list_append UInt8 close_bytes rest;
    open_rest : List UInt8 = list_append UInt8 child_bytes child_rest;
    open_starts : Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 open_bytes open_rest) =
      trans
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest)
        (list_append UInt8 open_bytes open_rest)
        starts_with
        (printed_not_suffix_view child rest);
    child_starts : Equal
      (List UInt8)
      (source_suffix after_open)
      (list_append UInt8 child_bytes child_rest) =
      source_suffix_after_codes cur open_bytes open_rest open_starts;
    close_starts : Equal
      (List UInt8)
      (source_suffix child_end)
      (list_append UInt8 close_bytes rest) =
      source_suffix_after_codes after_open child_bytes child_rest child_starts;
    strict_child : Equal Bool
      (cursor_nat_lt (byte_cursor_remaining after_open) (byte_cursor_remaining cur))
      True =
      cursor_after_nonempty_strict
        open_bytes
        cur
        open_rest
        open_starts
        not_open_encoded_nonempty;
    child_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (recur after_open)
      (Decoded ByteCursor Span (Syntax BoolExpr) child_syntax child_end) =
      recur_works
        after_open
        child_syntax
        child_end
        strict_child
        (printed_bool_spec_intro child after_open child_rest child_starts);
    open_succeeds : Equal
      (DecoderResult ByteCursor Span UInt8)
      (not_open_token_decoder cur)
      (Decoded ByteCursor Span UInt8 (list_last_byte open_bytes) after_open) =
      not_open_token_succeeds cur open_rest open_starts;
    opening_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder after_open)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) after_open) =
      spaces_on_printed_expr child after_open child_rest child_starts;
    closing_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder child_end)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) child_end) =
      spaces_on_close_token child_end rest close_starts;
    close_succeeds : Equal
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_decoder close_code child_end)
      (Decoded ByteCursor Span UInt8 (list_last_byte close_bytes) after_close) =
      close_byte_succeeds child_end rest close_starts;
    parsed_syntax : Syntax BoolExpr =
      syntax_node_unary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BNot (erase_spans child_syntax))
        child_syntax;
    parsed : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_not_decoder recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) parsed_syntax after_close) =
      bool_not_succeeds_on_parts
        recur
        cur
        after_open
        child_syntax
        child_end
        after_close
        (list_last_byte open_bytes)
        (list_last_byte close_bytes)
        open_succeeds
        opening_spaces
        child_succeeds
        closing_spaces
        close_succeeds;
    canonical_end : ByteCursor = printed_end cur (BNot child);
    end_matches : Equal ByteCursor after_close canonical_end =
      sym ByteCursor canonical_end after_close (printed_not_end_view cur child);
    child_value_matches : Equal BoolExpr (BNot (erase_spans child_syntax)) (BNot child) =
      cong
        BoolExpr
        BoolExpr
        (erase_spans child_syntax)
        child
        (λvalue. BNot value)
        (printed_syntax_erases child after_open);
    value_matches : Equal
      (Syntax BoolExpr)
      parsed_syntax
      (syntax_node_unary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BNot child)
        child_syntax) =
      cong
        BoolExpr
        (Syntax BoolExpr)
        (BNot (erase_spans child_syntax))
        (BNot child)
        (λvalue.
          syntax_node_unary
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position after_close)
            value
            child_syntax)
        child_value_matches;
    span_matches : Equal
      (Syntax BoolExpr)
      (syntax_node_unary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BNot child)
        child_syntax)
      (printed_syntax cur (BNot child)) =
      cong
        Nat
        (Syntax BoolExpr)
        (byte_cursor_position after_close)
        (byte_cursor_position canonical_end)
        (λposition.
          syntax_node_unary
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            position
            (BNot child)
            child_syntax)
        (cong ByteCursor Nat after_close canonical_end byte_cursor_position end_matches);
    syntax_matches : Equal (Syntax BoolExpr) parsed_syntax (printed_syntax cur (BNot child)) =
      trans
        (Syntax BoolExpr)
        parsed_syntax
        (syntax_node_unary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position after_close)
          (BNot child)
          child_syntax)
        (printed_syntax cur (BNot child))
        value_matches
        span_matches
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_not_decoder recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) parsed_syntax after_close)
      (Decoded
        ByteCursor
        Span
        (Syntax BoolExpr)
        (printed_syntax cur (BNot child))
        canonical_end)
      parsed
      (decoded_bool_result_transport
        parsed_syntax
        (printed_syntax cur (BNot child))
        after_close
        canonical_end
        syntax_matches
        end_matches)

theorem bool_layer_not_succeeds_printed
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (child : BoolExpr)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest))
      (recur_works : (inner : ByteCursor)
        → (syntax : Syntax BoolExpr)
        → (next : ByteCursor)
        → Equal
        Bool
        (cursor_nat_lt (byte_cursor_remaining inner) (byte_cursor_remaining cur))
        True
        → PrintedBoolSpec
        inner
        syntax
        next
        → Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur inner)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (printed_syntax cur (BNot child))
          (printed_end cur (BNot child))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode not_open_text);
    child_bytes : List UInt8 = bytes_to_list (print_bool_expr child);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    after_open : List UInt8 =
      list_append UInt8 child_bytes (list_append UInt8 close_bytes rest);
    open_starts : Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 open_bytes after_open) =
      trans
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BNot child))) rest)
        (list_append UInt8 open_bytes after_open)
        starts_with
        (printed_not_suffix_view child rest);
    open_codes : Equal
      (List Int)
      (map UInt8 Int uint8_to_int open_bytes)
      (Cons Int open_initial_code not_open_remaining_codes) =
      bytes_encode_ascii_octets not_open_text not_open_ascii;
    syntax : Syntax BoolExpr = printed_syntax cur (BNot child);
    next : ByteCursor = printed_end cur (BNot child)
  in
    bool_layer_open_success
      recur
      cur
      open_bytes
      after_open
      not_open_remaining_codes
      open_starts
      open_codes
      syntax
      next
      (decoder_alt_first_success
        ByteCursor
        Span
        (Syntax BoolExpr)
        (bool_not_decoder recur)
        (bool_and_decoder recur)
        cur
        syntax
        next
        (bool_not_decoder_succeeds_printed recur cur child rest starts_with recur_works))

fn bool_and_continue
      (cur : ByteCursor)
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (ignored : UInt8)
      (after_open : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  and_leading_after cur recur after_open

theorem bool_and_result_pointwise
      (recur : Decoder ByteCursor Span (Syntax BoolExpr)) (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_and_decoder recur cur)
        (decoder_then_result
          UInt8
          (Syntax BoolExpr)
          (and_open_token_decoder cur)
          (bool_and_continue cur recur)) =
  Refl

theorem bool_and_succeeds_on_parts
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (after_open : ByteCursor)
      (left_syntax : Syntax BoolExpr)
      (left_end : ByteCursor)
      (after_separator : ByteCursor)
      (right_syntax : Syntax BoolExpr)
      (right_end : ByteCursor)
      (after_close : ByteCursor)
      (open_byte : UInt8)
      (separator_byte : UInt8)
      (close_byte : UInt8)
      (open_succeeds : Equal
        (DecoderResult ByteCursor Span UInt8)
        (and_open_token_decoder cur)
        (Decoded ByteCursor Span UInt8 open_byte after_open))
      (opening_spaces : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder after_open)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) after_open))
      (left_succeeds : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur after_open)
        (Decoded ByteCursor Span (Syntax BoolExpr) left_syntax left_end))
      (separator_succeeds : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder separator_code left_end)
        (Decoded ByteCursor Span UInt8 separator_byte after_separator))
      (middle_spaces : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder after_separator)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) after_separator))
      (right_succeeds : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur after_separator)
        (Decoded ByteCursor Span (Syntax BoolExpr) right_syntax right_end))
      (closing_spaces : Equal
        (DecoderResult ByteCursor Span (List UInt8))
        (spaces_decoder right_end)
        (Decoded ByteCursor Span (List UInt8) (Nil UInt8) right_end))
      (close_succeeds : Equal
        (DecoderResult ByteCursor Span UInt8)
        (byte_code_decoder close_code right_end)
        (Decoded ByteCursor Span UInt8 close_byte after_close))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_and_decoder recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (syntax_node_binary
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position after_close)
            (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
            left_syntax
            right_syntax)
          after_close) =
  let
    parsed_syntax : Syntax BoolExpr =
      syntax_node_binary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
        left_syntax
        right_syntax;
    closed : DecoderResult ByteCursor Span (Syntax BoolExpr) =
      Decoded ByteCursor Span (Syntax BoolExpr) parsed_syntax after_close;
    close_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_close_after cur left_syntax right_syntax right_end)
      closed =
      decoder_then_success
        UInt8
        (Syntax BoolExpr)
        (byte_code_decoder close_code right_end)
        close_byte
        after_close
        (λignored.
          λend.
            Decoded
              ByteCursor
              Span
              (Syntax BoolExpr)
              (syntax_node_binary
                (byte_cursor_source cur)
                (byte_cursor_position cur)
                (byte_cursor_position end)
                (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
                left_syntax
                right_syntax)
              end)
        close_succeeds;
    trailing_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_trailing_after cur left_syntax right_syntax right_end)
      closed =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (and_trailing_after cur left_syntax right_syntax right_end)
        (and_close_after cur left_syntax right_syntax right_end)
        closed
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder right_end)
          (Nil UInt8)
          right_end
          (λignored. λclose_start. and_close_after cur left_syntax right_syntax close_start)
          closing_spaces)
        close_stage;
    right_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_right_after cur recur left_syntax after_separator)
      closed =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (and_right_after cur recur left_syntax after_separator)
        (and_trailing_after cur left_syntax right_syntax right_end)
        closed
        (decoder_then_success
          (Syntax BoolExpr)
          (Syntax BoolExpr)
          (recur after_separator)
          right_syntax
          right_end
          (λright_value. λend. and_trailing_after cur left_syntax right_value end)
          right_succeeds)
        trailing_stage;
    middle_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_middle_after cur recur left_syntax after_separator)
      closed =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (and_middle_after cur recur left_syntax after_separator)
        (and_right_after cur recur left_syntax after_separator)
        closed
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder after_separator)
          (Nil UInt8)
          after_separator
          (λignored. λright_start. and_right_after cur recur left_syntax right_start)
          middle_spaces)
        right_stage;
    separator_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_separator_after cur recur left_syntax left_end)
      closed =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (and_separator_after cur recur left_syntax left_end)
        (and_middle_after cur recur left_syntax after_separator)
        closed
        (decoder_then_success
          UInt8
          (Syntax BoolExpr)
          (byte_code_decoder separator_code left_end)
          separator_byte
          after_separator
          (λignored. λseparator_end. and_middle_after cur recur left_syntax separator_end)
          separator_succeeds)
        middle_stage;
    left_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_left_after cur recur after_open)
      closed =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (and_left_after cur recur after_open)
        (and_separator_after cur recur left_syntax left_end)
        closed
        (decoder_then_success
          (Syntax BoolExpr)
          (Syntax BoolExpr)
          (recur after_open)
          left_syntax
          left_end
          (λleft_value. λend. and_separator_after cur recur left_value end)
          left_succeeds)
        separator_stage;
    leading_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (and_leading_after cur recur after_open)
      closed =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (and_leading_after cur recur after_open)
        (and_left_after cur recur after_open)
        closed
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder after_open)
          (Nil UInt8)
          after_open
          (λignored. λleft_start. and_left_after cur recur left_start)
          opening_spaces)
        left_stage
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_and_decoder recur cur)
      (and_leading_after cur recur after_open)
      closed
      (trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_and_decoder recur cur)
        (decoder_then_result
          UInt8
          (Syntax BoolExpr)
          (and_open_token_decoder cur)
          (bool_and_continue cur recur))
        (and_leading_after cur recur after_open)
        (bool_and_result_pointwise recur cur)
        (decoder_then_success
          UInt8
          (Syntax BoolExpr)
          (and_open_token_decoder cur)
          open_byte
          after_open
          (bool_and_continue cur recur)
          open_succeeds))
      leading_stage

theorem bool_and_decoder_succeeds_printed
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (left : BoolExpr)
      (right : BoolExpr)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest))
      (recur_works : (inner : ByteCursor)
        → (syntax : Syntax BoolExpr)
        → (next : ByteCursor)
        → Equal
        Bool
        (cursor_nat_lt (byte_cursor_remaining inner) (byte_cursor_remaining cur))
        True
        → PrintedBoolSpec
        inner
        syntax
        next
        → Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur inner)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_and_decoder recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (printed_syntax cur (BAnd left right))
          (printed_end cur (BAnd left right))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode and_open_text);
    left_bytes : List UInt8 = bytes_to_list (print_bool_expr left);
    separator_bytes : List UInt8 = bytes_to_list (bytes_encode separator_text);
    right_bytes : List UInt8 = bytes_to_list (print_bool_expr right);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    after_open : ByteCursor = cursor_after_codes cur open_bytes;
    left_syntax : Syntax BoolExpr = printed_syntax after_open left;
    left_end : ByteCursor = printed_end after_open left;
    after_separator : ByteCursor = cursor_after_codes left_end separator_bytes;
    right_syntax : Syntax BoolExpr = printed_syntax after_separator right;
    right_end : ByteCursor = printed_end after_separator right;
    after_close : ByteCursor = cursor_after_codes right_end close_bytes;
    right_rest : List UInt8 = list_append UInt8 close_bytes rest;
    separator_rest : List UInt8 = list_append UInt8 right_bytes right_rest;
    left_rest : List UInt8 = list_append UInt8 separator_bytes separator_rest;
    open_rest : List UInt8 = list_append UInt8 left_bytes left_rest;
    open_starts : Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 open_bytes open_rest) =
      trans
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest)
        (list_append UInt8 open_bytes open_rest)
        starts_with
        (printed_and_suffix_view left right rest);
    left_starts : Equal
      (List UInt8)
      (source_suffix after_open)
      (list_append UInt8 left_bytes left_rest) =
      source_suffix_after_codes cur open_bytes open_rest open_starts;
    separator_starts : Equal
      (List UInt8)
      (source_suffix left_end)
      (list_append UInt8 separator_bytes separator_rest) =
      source_suffix_after_codes after_open left_bytes left_rest left_starts;
    right_starts : Equal
      (List UInt8)
      (source_suffix after_separator)
      (list_append UInt8 right_bytes right_rest) =
      source_suffix_after_codes left_end separator_bytes separator_rest separator_starts;
    close_starts : Equal
      (List UInt8)
      (source_suffix right_end)
      (list_append UInt8 close_bytes rest) =
      source_suffix_after_codes after_separator right_bytes right_rest right_starts;
    strict_open : Equal Bool
      (cursor_nat_lt (byte_cursor_remaining after_open) (byte_cursor_remaining cur))
      True =
      cursor_after_nonempty_strict
        open_bytes
        cur
        open_rest
        open_starts
        and_open_encoded_nonempty;
    strict_left : Equal Bool
      (cursor_nat_lt (byte_cursor_remaining left_end) (byte_cursor_remaining after_open))
      True =
      cursor_after_nonempty_strict
        left_bytes
        after_open
        left_rest
        left_starts
        (print_bool_expr_nonempty left);
    strict_separator : Equal Bool
      (cursor_nat_lt (byte_cursor_remaining after_separator) (byte_cursor_remaining left_end))
      True =
      cursor_after_nonempty_strict
        separator_bytes
        left_end
        separator_rest
        separator_starts
        separator_encoded_nonempty;
    strict_right : Equal Bool
      (cursor_nat_lt (byte_cursor_remaining after_separator) (byte_cursor_remaining cur))
      True =
      cursor_nat_lt_trans
        (byte_cursor_remaining after_separator)
        (byte_cursor_remaining left_end)
        (byte_cursor_remaining cur)
        strict_separator
        (cursor_nat_lt_trans
          (byte_cursor_remaining left_end)
          (byte_cursor_remaining after_open)
          (byte_cursor_remaining cur)
          strict_left
          strict_open);
    left_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (recur after_open)
      (Decoded ByteCursor Span (Syntax BoolExpr) left_syntax left_end) =
      recur_works
        after_open
        left_syntax
        left_end
        strict_open
        (printed_bool_spec_intro left after_open left_rest left_starts);
    right_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (recur after_separator)
      (Decoded ByteCursor Span (Syntax BoolExpr) right_syntax right_end) =
      recur_works
        after_separator
        right_syntax
        right_end
        strict_right
        (printed_bool_spec_intro right after_separator right_rest right_starts);
    open_succeeds : Equal
      (DecoderResult ByteCursor Span UInt8)
      (and_open_token_decoder cur)
      (Decoded ByteCursor Span UInt8 (list_last_byte open_bytes) after_open) =
      and_open_token_succeeds cur open_rest open_starts;
    opening_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder after_open)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) after_open) =
      spaces_on_printed_expr left after_open left_rest left_starts;
    separator_succeeds : Equal
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_decoder separator_code left_end)
      (Decoded ByteCursor Span UInt8 (list_last_byte separator_bytes) after_separator) =
      separator_byte_succeeds left_end separator_rest separator_starts;
    middle_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder after_separator)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) after_separator) =
      spaces_on_printed_expr right after_separator right_rest right_starts;
    closing_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder right_end)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) right_end) =
      spaces_on_close_token right_end rest close_starts;
    close_succeeds : Equal
      (DecoderResult ByteCursor Span UInt8)
      (byte_code_decoder close_code right_end)
      (Decoded ByteCursor Span UInt8 (list_last_byte close_bytes) after_close) =
      close_byte_succeeds right_end rest close_starts;
    parsed_syntax : Syntax BoolExpr =
      syntax_node_binary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
        left_syntax
        right_syntax;
    parsed : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_and_decoder recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) parsed_syntax after_close) =
      bool_and_succeeds_on_parts
        recur
        cur
        after_open
        left_syntax
        left_end
        after_separator
        right_syntax
        right_end
        after_close
        (list_last_byte open_bytes)
        (list_last_byte separator_bytes)
        (list_last_byte close_bytes)
        open_succeeds
        opening_spaces
        left_succeeds
        separator_succeeds
        middle_spaces
        right_succeeds
        closing_spaces
        close_succeeds;
    canonical_end : ByteCursor = printed_end cur (BAnd left right);
    end_matches : Equal ByteCursor after_close canonical_end =
      sym ByteCursor canonical_end after_close (printed_and_end_view cur left right);
    left_value_matches : Equal BoolExpr
      (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
      (BAnd left (erase_spans right_syntax)) =
      cong
        BoolExpr
        BoolExpr
        (erase_spans left_syntax)
        left
        (λvalue. BAnd value (erase_spans right_syntax))
        (printed_syntax_erases left after_open);
    right_value_matches : Equal BoolExpr
      (BAnd left (erase_spans right_syntax))
      (BAnd left right) =
      cong
        BoolExpr
        BoolExpr
        (erase_spans right_syntax)
        right
        (λvalue. BAnd left value)
        (printed_syntax_erases right after_separator);
    expr_matches : Equal BoolExpr
      (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
      (BAnd left right) =
      trans
        BoolExpr
        (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
        (BAnd left (erase_spans right_syntax))
        (BAnd left right)
        left_value_matches
        right_value_matches;
    value_matches : Equal
      (Syntax BoolExpr)
      parsed_syntax
      (syntax_node_binary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BAnd left right)
        left_syntax
        right_syntax) =
      cong
        BoolExpr
        (Syntax BoolExpr)
        (BAnd (erase_spans left_syntax) (erase_spans right_syntax))
        (BAnd left right)
        (λvalue.
          syntax_node_binary
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            (byte_cursor_position after_close)
            value
            left_syntax
            right_syntax)
        expr_matches;
    span_matches : Equal
      (Syntax BoolExpr)
      (syntax_node_binary
        (byte_cursor_source cur)
        (byte_cursor_position cur)
        (byte_cursor_position after_close)
        (BAnd left right)
        left_syntax
        right_syntax)
      (printed_syntax cur (BAnd left right)) =
      cong
        Nat
        (Syntax BoolExpr)
        (byte_cursor_position after_close)
        (byte_cursor_position canonical_end)
        (λposition.
          syntax_node_binary
            (byte_cursor_source cur)
            (byte_cursor_position cur)
            position
            (BAnd left right)
            left_syntax
            right_syntax)
        (cong ByteCursor Nat after_close canonical_end byte_cursor_position end_matches);
    syntax_matches : Equal
      (Syntax BoolExpr)
      parsed_syntax
      (printed_syntax cur (BAnd left right)) =
      trans
        (Syntax BoolExpr)
        parsed_syntax
        (syntax_node_binary
          (byte_cursor_source cur)
          (byte_cursor_position cur)
          (byte_cursor_position after_close)
          (BAnd left right)
          left_syntax
          right_syntax)
        (printed_syntax cur (BAnd left right))
        value_matches
        span_matches
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_and_decoder recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) parsed_syntax after_close)
      (Decoded
        ByteCursor
        Span
        (Syntax BoolExpr)
        (printed_syntax cur (BAnd left right))
        canonical_end)
      parsed
      (decoded_bool_result_transport
        parsed_syntax
        (printed_syntax cur (BAnd left right))
        after_close
        canonical_end
        syntax_matches
        end_matches)

theorem decoder_seq_second_rejected
      (c : Type)
      (loc : Type)
      (a : Type)
      (b : Type)
      (first : Decoder c loc a)
      (second : Decoder c loc b)
      (cur : c)
      (value : a)
      (mid : c)
      (at : loc)
      (first_ok : Equal (DecoderResult c loc a) (first cur) (Decoded c loc a value mid))
      (second_rejected : Equal
        (DecoderResult c loc b)
        (second mid)
        (DecoderFailed c loc b (DecoderRejected loc at)))
    : Equal
        (DecoderResult c loc b)
        (decoder_seq c loc a b first second cur)
        (DecoderFailed c loc b (DecoderRejected loc at)) =
  trans
    (DecoderResult c loc b)
    (decoder_seq c loc a b first second cur)
    (decoder_seq_resume c loc a b second (first cur))
    (DecoderFailed c loc b (DecoderRejected loc at))
    (decoder_seq_resume_equation c loc a b first second cur)
    (trans
      (DecoderResult c loc b)
      (decoder_seq_resume c loc a b second (first cur))
      (decoder_seq_resume c loc a b second (Decoded c loc a value mid))
      (DecoderFailed c loc b (DecoderRejected loc at))
      (cong
        (DecoderResult c loc a)
        (DecoderResult c loc b)
        (first cur)
        (Decoded c loc a value mid)
        (decoder_seq_resume c loc a b second)
        first_ok)
      second_rejected)

theorem not_open_token_rejects_and_prefix
      (prefix : List UInt8)
    : (cur : ByteCursor)
      → (rest : List UInt8)
      → Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest)
      → Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int open_initial_code (Cons Int and_second_code and_open_after_second_codes))
      → Equal
        (DecoderResult ByteCursor Span UInt8)
        (not_open_token_decoder cur)
        (DecoderFailed
          ByteCursor
          Span
          UInt8
          (DecoderRejected Span (byte_cursor_locate (byte_cursor_advance cur)))) =
  match prefix {
    Nil ↦ λcur. λrest. λstarts_with. λmapped. absurd mapped;
    Cons first tail ↦
      λcur.
        λrest.
          λstarts_with.
            λmapped.
              let
                actual_tail : List Int = Cons Int and_second_code and_open_after_second_codes;
                head_matches : Equal Int (uint8_to_int first) open_initial_code =
                  mapped_codes_cons_head first tail open_initial_code actual_tail mapped;
                tail_matches : Equal (List Int) (map UInt8 Int uint8_to_int tail) actual_tail =
                  mapped_codes_cons_tail first tail open_initial_code actual_tail mapped;
                after_first : ByteCursor = byte_cursor_advance cur;
                remaining : List UInt8 = list_append UInt8 tail rest;
                first_succeeds : Equal
                  (DecoderResult ByteCursor Span UInt8)
                  (byte_code_decoder open_initial_code cur)
                  (Decoded ByteCursor Span UInt8 first after_first) =
                  byte_code_success_from_head
                    cur
                    open_initial_code
                    first
                    remaining
                    starts_with
                    head_matches;
                tail_starts : Equal (List UInt8) (source_suffix after_first) remaining =
                  source_suffix_after_codes
                    cur
                    (Cons UInt8 first (Nil UInt8))
                    remaining
                    starts_with;
                second_rejected : Equal
                  (DecoderResult ByteCursor Span UInt8)
                  (byte_code_decoder not_second_code after_first)
                  (DecoderFailed
                    ByteCursor
                    Span
                    UInt8
                    (DecoderRejected Span (byte_cursor_locate after_first))) =
                  byte_code_rejects_different_prefix
                    after_first
                    not_second_code
                    and_second_code
                    and_open_after_second_codes
                    tail
                    rest
                    (λsame. absurd same)
                    tail_starts
                    tail_matches;
                rest_rejected : Equal
                  (DecoderResult ByteCursor Span UInt8)
                  (token_codes_decoder not_open_remaining_codes after_first)
                  (DecoderFailed
                    ByteCursor
                    Span
                    UInt8
                    (DecoderRejected Span (byte_cursor_locate after_first))) =
                  decoder_seq_first_rejected
                    ByteCursor
                    Span
                    UInt8
                    UInt8
                    (byte_code_decoder not_second_code)
                    (token_codes_decoder (Cons Int 111 (Cons Int 116 (Cons Int 32 (Nil Int)))))
                    after_first
                    (byte_cursor_locate after_first)
                    second_rejected
              in
                trans
                  (DecoderResult ByteCursor Span UInt8)
                  (not_open_token_decoder cur)
                  (token_codes_decoder
                    (map Char Int charToInt (string_to_list_char not_open_text))
                    cur)
                  (DecoderFailed
                    ByteCursor
                    Span
                    UInt8
                    (DecoderRejected Span (byte_cursor_locate after_first)))
                  (not_open_token_code_decoder_pointwise cur)
                  (decoder_seq_second_rejected
                    ByteCursor
                    Span
                    UInt8
                    UInt8
                    (byte_code_decoder open_initial_code)
                    (token_codes_decoder not_open_remaining_codes)
                    cur
                    first
                    after_first
                    (byte_cursor_locate after_first)
                    first_succeeds
                    rest_rejected)
  }

theorem bool_not_rejects_and_prefix
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (prefix : List UInt8)
      (rest : List UInt8)
      (starts_with : Equal (List UInt8) (source_suffix cur) (list_append UInt8 prefix rest))
      (mapped : Equal
        (List Int)
        (map UInt8 Int uint8_to_int prefix)
        (Cons Int open_initial_code (Cons Int and_second_code and_open_after_second_codes)))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_not_decoder recur cur)
        (DecoderFailed
          ByteCursor
          Span
          (Syntax BoolExpr)
          (DecoderRejected Span (byte_cursor_locate (byte_cursor_advance cur)))) =
  trans
    (DecoderResult ByteCursor Span (Syntax BoolExpr))
    (bool_not_decoder recur cur)
    (decoder_then_result
      UInt8
      (Syntax BoolExpr)
      (not_open_token_decoder cur)
      (bool_not_continue cur recur))
    (DecoderFailed
      ByteCursor
      Span
      (Syntax BoolExpr)
      (DecoderRejected Span (byte_cursor_locate (byte_cursor_advance cur))))
    (bool_not_result_pointwise recur cur)
    (cong
      (DecoderResult ByteCursor Span UInt8)
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (not_open_token_decoder cur)
      (DecoderFailed
        ByteCursor
        Span
        UInt8
        (DecoderRejected Span (byte_cursor_locate (byte_cursor_advance cur))))
      (λobserved.
        decoder_then_result UInt8 (Syntax BoolExpr) observed (bool_not_continue cur recur))
      (not_open_token_rejects_and_prefix prefix cur rest starts_with mapped))

theorem bool_layer_and_succeeds_printed
      (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      (cur : ByteCursor)
      (left : BoolExpr)
      (right : BoolExpr)
      (rest : List UInt8)
      (starts_with : Equal
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest))
      (recur_works : (inner : ByteCursor)
        → (syntax : Syntax BoolExpr)
        → (next : ByteCursor)
        → Equal
        Bool
        (cursor_nat_lt (byte_cursor_remaining inner) (byte_cursor_remaining cur))
        True
        → PrintedBoolSpec
        inner
        syntax
        next
        → Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (recur inner)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded
          ByteCursor
          Span
          (Syntax BoolExpr)
          (printed_syntax cur (BAnd left right))
          (printed_end cur (BAnd left right))) =
  let
    open_bytes : List UInt8 = bytes_to_list (bytes_encode and_open_text);
    left_bytes : List UInt8 = bytes_to_list (print_bool_expr left);
    separator_bytes : List UInt8 = bytes_to_list (bytes_encode separator_text);
    right_bytes : List UInt8 = bytes_to_list (print_bool_expr right);
    close_bytes : List UInt8 = bytes_to_list (bytes_encode close_text);
    open_rest : List UInt8 =
      list_append
        UInt8
        left_bytes
        (list_append
          UInt8
          separator_bytes
          (list_append UInt8 right_bytes (list_append UInt8 close_bytes rest)));
    open_starts : Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 open_bytes open_rest) =
      trans
        (List UInt8)
        (source_suffix cur)
        (list_append UInt8 (bytes_to_list (print_bool_expr (BAnd left right))) rest)
        (list_append UInt8 open_bytes open_rest)
        starts_with
        (printed_and_suffix_view left right rest);
    open_codes : Equal
      (List Int)
      (map UInt8 Int uint8_to_int open_bytes)
      (Cons Int open_initial_code (Cons Int and_second_code and_open_after_second_codes)) =
      bytes_encode_ascii_octets and_open_text and_open_ascii;
    syntax : Syntax BoolExpr = printed_syntax cur (BAnd left right);
    next : ByteCursor = printed_end cur (BAnd left right);
    and_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_and_decoder recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
      bool_and_decoder_succeeds_printed recur cur left right rest starts_with recur_works;
    not_rejected : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_not_decoder recur cur)
      (DecoderFailed
        ByteCursor
        Span
        (Syntax BoolExpr)
        (DecoderRejected Span (byte_cursor_locate (byte_cursor_advance cur)))) =
      bool_not_rejects_and_prefix recur cur open_bytes open_rest open_starts open_codes;
    last_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_layer_last recur cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_layer_last recur cur)
        (bool_and_decoder recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
        (decoder_alt_rejection_uses_second
          ByteCursor
          Span
          (Syntax BoolExpr)
          (bool_not_decoder recur)
          (bool_and_decoder recur)
          cur
          (byte_cursor_locate (byte_cursor_advance cur))
          not_rejected)
        and_succeeds
  in
    bool_layer_open_success
      recur
      cur
      open_bytes
      open_rest
      (Cons Int and_second_code and_open_after_second_codes)
      open_starts
      open_codes
      syntax
      next
      last_succeeds

theorem bool_layer_succeeds_for_expr
      (e : BoolExpr)
    : (recur : Decoder ByteCursor Span (Syntax BoolExpr))
      → (cur : ByteCursor)
      → ((inner : ByteCursor)
          → (syntax : Syntax BoolExpr)
          → (next : ByteCursor)
          → Equal
          Bool
          (cursor_nat_lt (byte_cursor_remaining inner) (byte_cursor_remaining cur))
          True
          → PrintedBoolSpec
          inner
          syntax
          next
          → Equal
          (DecoderResult ByteCursor Span (Syntax BoolExpr))
          (recur inner)
          (Decoded ByteCursor Span (Syntax BoolExpr) syntax next))
      → (syntax : Syntax BoolExpr)
      → (next : ByteCursor)
      → PrintedBoolSpecAt e cur syntax next
      → Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_decoder_layer recur cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
  match e {
    BTrue ↦
      λrecur.
        λcur.
          λrecur_works.
            λsyntax.
              λnext. λholds. bool_layer_true_for_printed_spec recur cur syntax next holds;
    BFalse ↦
      λrecur.
        λcur.
          λrecur_works.
            λsyntax.
              λnext. λholds. bool_layer_false_for_printed_spec recur cur syntax next holds;
    BNot child ↦
      λrecur.
        λcur.
          λrecur_works.
            λsyntax.
              λnext.
                λholds.
                  bool_layer_from_canonical
                    (BNot child)
                    recur
                    cur
                    syntax
                    next
                    holds
                    (bool_layer_not_succeeds_printed
                      recur
                      cur
                      child
                      (source_suffix next)
                      (printed_bool_spec_prefix (BNot child) cur syntax next holds)
                      recur_works);
    BAnd left right ↦
      λrecur.
        λcur.
          λrecur_works.
            λsyntax.
              λnext.
                λholds.
                  bool_layer_from_canonical
                    (BAnd left right)
                    recur
                    cur
                    syntax
                    next
                    holds
                    (bool_layer_and_succeeds_printed
                      recur
                      cur
                      left
                      right
                      (source_suffix next)
                      (printed_bool_spec_prefix (BAnd left right) cur syntax next holds)
                      recur_works)
  }

theorem bool_expression_decoder_succeeds_printed
      (cur : ByteCursor)
      (syntax : Syntax BoolExpr)
      (next : ByteCursor)
      (holds : PrintedBoolSpec cur syntax next)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (bool_expression_decoder cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
  decoder_recursive_succeeds
    ByteCursor
    UInt8
    Span
    (Syntax BoolExpr)
    byte_cursor_ops
    bool_decoder_layer
    PrintedBoolSpec
    (λrecur.
      λgrammar_start.
        λrecur_works.
          λvalue.
            λend.
              λspec_holds.
                bool_layer_succeeds_for_expr
                  (erase_spans value)
                  recur
                  grammar_start
                  recur_works
                  value
                  end
                  spec_holds)
    cur
    syntax
    next
    holds

theorem complete_bool_finish_on_empty
      (syntax : Syntax BoolExpr)
      (end : ByteCursor)
      (empty : Equal (List UInt8) (source_suffix end) (Nil UInt8))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (complete_bool_finish syntax end)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax end) =
  let remaining_zero : Equal Nat (byte_cursor_remaining end) Zero =
    trans
      Nat
      (byte_cursor_remaining end)
      (length UInt8 (source_suffix end))
      Zero
      (sym
        Nat
        (length UInt8 (source_suffix end))
        (byte_cursor_remaining end)
        (source_suffix_length end))
      (cong (List UInt8) Nat (source_suffix end) (Nil UInt8) (length UInt8) empty)
  in
    cong
      Nat
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (byte_cursor_remaining end)
      Zero
      (λremaining. complete_bool_finish_result syntax end remaining)
      remaining_zero

fn complete_bool_continue
      (ignored : List UInt8) (start : ByteCursor)
    : DecoderResult ByteCursor Span (Syntax BoolExpr) =
  complete_bool_expression_after start

theorem complete_bool_decoder_pointwise
      (cur : ByteCursor)
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (complete_bool_decoder cur)
        (decoder_then_result
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder cur)
          complete_bool_continue) =
  Refl

theorem complete_bool_decoder_succeeds_printed_cursor
      (cur : ByteCursor)
      (e : BoolExpr)
      (printed_input : Equal
        (List UInt8)
        (source_suffix cur)
        (bytes_to_list (print_bool_expr e)))
    : Equal
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (complete_bool_decoder cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) (printed_syntax cur e) (printed_end cur e)) =
  let
    printed_bytes : List UInt8 = bytes_to_list (print_bool_expr e);
    syntax : Syntax BoolExpr = printed_syntax cur e;
    next : ByteCursor = printed_end cur e;
    with_empty_tail : Equal
      (List UInt8)
      (source_suffix cur)
      (list_append UInt8 printed_bytes (Nil UInt8)) =
      trans
        (List UInt8)
        (source_suffix cur)
        printed_bytes
        (list_append UInt8 printed_bytes (Nil UInt8))
        printed_input
        (sym
          (List UInt8)
          (list_append UInt8 printed_bytes (Nil UInt8))
          printed_bytes
          ((proof right_unit for list_append) UInt8 printed_bytes));
    empty_after_print : Equal (List UInt8) (source_suffix next) (Nil UInt8) =
      source_suffix_after_codes cur printed_bytes (Nil UInt8) with_empty_tail;
    starting_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder cur)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) cur) =
      spaces_on_printed_expr e cur (Nil UInt8) with_empty_tail;
    expression_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (bool_expression_decoder cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
      bool_expression_decoder_succeeds_printed
        cur
        syntax
        next
        (printed_bool_spec_intro e cur (Nil UInt8) with_empty_tail);
    ending_spaces : Equal
      (DecoderResult ByteCursor Span (List UInt8))
      (spaces_decoder next)
      (Decoded ByteCursor Span (List UInt8) (Nil UInt8) next) =
      spaces_on_empty_suffix next empty_after_print;
    finish_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (complete_bool_finish syntax next)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
      complete_bool_finish_on_empty syntax next empty_after_print;
    trailing_succeeds : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (complete_bool_trailing_after syntax next)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (complete_bool_trailing_after syntax next)
        (complete_bool_finish syntax next)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder next)
          (Nil UInt8)
          next
          (λignored. λend. complete_bool_finish syntax end)
          ending_spaces)
        finish_succeeds;
    expression_stage : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (complete_bool_expression_after cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next) =
      trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (complete_bool_expression_after cur)
        (complete_bool_trailing_after syntax next)
        (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
        (decoder_then_success
          (Syntax BoolExpr)
          (Syntax BoolExpr)
          (bool_expression_decoder cur)
          syntax
          next
          complete_bool_trailing_after
          expression_succeeds)
        trailing_succeeds
  in
    trans
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (complete_bool_decoder cur)
      (complete_bool_expression_after cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) syntax next)
      (trans
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (complete_bool_decoder cur)
        (decoder_then_result
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder cur)
          complete_bool_continue)
        (complete_bool_expression_after cur)
        (complete_bool_decoder_pointwise cur)
        (decoder_then_success
          (List UInt8)
          (Syntax BoolExpr)
          (spaces_decoder cur)
          (Nil UInt8)
          cur
          complete_bool_continue
          starting_spaces))
      expression_stage

fn parsed_bool_outcome
      (s : Source) (start : Nat) (outcome : DecoderResult ByteCursor Span (Syntax BoolExpr))
    : ParseResult (Syntax BoolExpr) =
  match outcome {
    Decoded syntax next ↦
      Parsed
        (Syntax BoolExpr)
        syntax
        (MkSpan start (byte_cursor_position next))
        (byte_cursor_position next);
    DecoderFailed err ↦ Failed (Syntax BoolExpr) (decoder_parse_error s err)
  }

theorem parse_bool_expr_result_pointwise
      (s : Source) (start : Nat) (h : LessEqNat start (source_length s))
    : Equal
        (ParseResult (Syntax BoolExpr))
        (parse_bool_expr s start h)
        (parsed_bool_outcome s start (complete_bool_decoder (MkByteCursor s start))) =
  Refl

theorem parse_bool_expr_succeeds_printed_source
      (s : Source)
      (e : BoolExpr)
      (h : LessEqNat Zero (source_length s))
      (source_is_printed : Equal Bytes (source_bytes s) (print_bool_expr e))
    : Equal
        (ParseResult (Syntax BoolExpr))
        (parse_bool_expr s Zero h)
        (Parsed
          (Syntax BoolExpr)
          (printed_syntax (MkByteCursor s Zero) e)
          (MkSpan Zero (byte_cursor_position (printed_end (MkByteCursor s Zero) e)))
          (byte_cursor_position (printed_end (MkByteCursor s Zero) e))) =
  let
    cur : ByteCursor = MkByteCursor s Zero;
    printed_bytes : List UInt8 = bytes_to_list (print_bool_expr e);
    printed_input : Equal (List UInt8) (source_suffix cur) printed_bytes =
      cong
        Bytes
        (List UInt8)
        (source_bytes s)
        (print_bool_expr e)
        bytes_to_list
        source_is_printed;
    decoded : Equal
      (DecoderResult ByteCursor Span (Syntax BoolExpr))
      (complete_bool_decoder cur)
      (Decoded ByteCursor Span (Syntax BoolExpr) (printed_syntax cur e) (printed_end cur e)) =
      complete_bool_decoder_succeeds_printed_cursor cur e printed_input
  in
    trans
      (ParseResult (Syntax BoolExpr))
      (parse_bool_expr s Zero h)
      (parsed_bool_outcome s Zero (complete_bool_decoder cur))
      (Parsed
        (Syntax BoolExpr)
        (printed_syntax cur e)
        (MkSpan Zero (byte_cursor_position (printed_end cur e)))
        (byte_cursor_position (printed_end cur e)))
      (parse_bool_expr_result_pointwise s Zero h)
      (cong
        (DecoderResult ByteCursor Span (Syntax BoolExpr))
        (ParseResult (Syntax BoolExpr))
        (complete_bool_decoder cur)
        (Decoded ByteCursor Span (Syntax BoolExpr) (printed_syntax cur e) (printed_end cur e))
        (parsed_bool_outcome s Zero)
        decoded)
```

These checked examples exercise the private Boolean comparison and syntax-leaf
constructor in their defining module. Clients use the public parser and syntax
accessors rather than importing either helper.

```ken example
const parsing_example_bool_expr_eq_same : Bool =
  bool_expr_eq (BAnd BTrue (BNot BFalse)) (BAnd BTrue (BNot BFalse))

const parsing_example_bool_expr_eq_other : Bool =
  bool_expr_eq (BAnd BTrue (BNot BFalse)) (BAnd BFalse (BNot BFalse))

fn parsing_example_syntax_leaf (s : Source) : Syntax BoolExpr =
  syntax_leaf s Zero (Suc Zero) BTrue
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
   `parser_from_decoder_laws`, `parser_pure_laws`, `parser_fail_laws`,
   `BoolExpr`, `BTrue`, `BFalse`, `BNot`, `BAnd`, `Syntax`, `MkSyntax`,
   `syntax_root`, `syntax_children`, `erase_spans`, `ValidLocatedList`,
   `ValidSyntax`, `parse_bool_expr`, `parse_bool_expr_total`,
   `parse_bool_expr_laws`, `print_bool_expr`, `print_bool_expr_utf8`,
   `ParsedPrintedBool`, `parse_bool_expr_print_round_trip`,
   `format_bool_expr`, `format_bool_expr_print_round_trip`,
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
4. **`trusted_base()` delta.** **Zero locally.** The byte bridges inherit
   `bytes_concat_list_view` (F1), `bytes_encode_ascii_octets` (F2), and
   `ascii_bytes_utf8` (F4') from `Data.Binary.BytesPrimitiveContracts`;
   the imported module registers four trusted facts, including
   `bytes_decode_encode` (F3). The grammar's byte-level inverse does not
   require a decode step; an independent client exercises F3 directly.
   Every proof defined in this package —
   `LessEqNat::refl`, `LessEqNat::zero_left`, `valid_zero_width_span`,
   `parse_bool_expr_total`, `parse_bool_expr_laws`,
   `parser_from_decoder_laws`, `parser_pure_laws`, `parser_fail_laws`,
   `print_bool_expr_utf8`, `parse_bool_expr_print_round_trip`,
   `format_bool_expr_print_round_trip`,
   `format_bool_expr_on_parse_success`,
   `format_bool_expr_on_parse_failure`, `ascii_encoded_byte_view`,
   `ascii_encoded_utf8`, and `ascii_concat_utf8` — is real and
   kernel-checked; no law or predicate is postulated.
5. **Proof families.** `LessEqNat::refl` — induction on `n`.
   `LessEqNat::zero_left` — definitional (first match arm). `valid_zero_width_span`
   — direct composition of the two via `and_intro`, no case-split of its
   own. `parse_bool_expr_total` — exhaustive parse-result split.
   `parser_from_decoder_laws` — decoder outcome bounds imply parser validity;
   exhaustive results give totality, and validity implies source locality.
   `parser_pure_laws` and `parser_fail_laws` — respectively preserve the input
   cursor and locate a zero-width failure at that cursor.
   `parse_bool_expr_laws` — instantiates the generic law with the checked
   Boolean decoder bound, source-bounded cursor, and private validity bridge;
   `decoder_recursive_preserves`
   handles fuel internally. `format_bool_expr_on_parse_success` and
   `format_bool_expr_on_parse_failure` — equality transport across each
   parser-result alternative. `ascii_encoded_byte_view` transports F2
   through the existing UInt8 retraction and byte-list injectivity;
   `ascii_encoded_byte_codes` transports the checked ASCII witness through
   F2, while `ascii_concat_utf8` transports its append over F1 into F4'.
   `print_bool_expr_utf8` supplies the `Source` field for any printed
   expression. The cursor-relative proof uses `decoder_many_rejected_succeeds`
   for absent spaces and `decoder_recursive_succeeds` for strict child
   progress; `ParsedPrintedBool` excludes parse failure and requires the
   parsed expression to erase to the printer's input. The formatter law
   additionally proves the original bytes are returned.
6. **Consumers.** Source-aware parser implementations can use this package's
   source, span, result, and validity vocabulary. Clients can construct a
   `Source` from `print_bool_expr_utf8`, apply the generic parser round trip,
   and apply the formatter law without assuming parse success.
7. **Validation evidence.** The catalog checks the
   `Source`/`Span`/`Located`/`ParseResult`/`Parser` surface, its zero local
   `trusted_base()` delta, the Boolean grammar's constructors and byte-token
   matching, and the absence of an exported unguarded repetition combinator.
   An independent client checks the exported proof on a source backed by
   printed bytes, alongside its separate ASCII-literal byte contracts.
