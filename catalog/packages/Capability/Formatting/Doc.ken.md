# `Capability.Formatting.Doc` — a small lawful document algebra

`Doc` separates inert text and layout choices from deterministic rendering.
Text leaves are structural `List Char` values, so fitting and the content laws
remain ordinary kernel-checked Ken.

## Contents

1. [Document algebra](#1-document-algebra)
2. [Fitting and rendering](#2-fitting-and-rendering)
3. [Content laws](#3-content-laws)
4. [String boundary](#4-string-boundary)
5. [Trust and derivation](#5-trust-and-derivation)

## 1. Document algebra

The algebra is closed at six constructors. `Text` stores characters directly;
`Line` is a soft separator; `Concat` composes; `Nest` controls indentation;
`Group` chooses flat or broken layout; and `Alt` offers two layouts of the same
content.

```ken
import Core.Classes.LawfulClasses (leq_nat)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (cong, sym, trans)

import Data.Collections.Derived (length, list_append)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (leq_nat_add_left_bound, leq_nat_add_right_bound)

data Doc : Type where {
  Text : List Char → Doc;
  Line : Doc;
  Concat : Doc → Doc → Doc;
  Nest : Nat → Doc → Doc;
  Group : Doc → Doc;
  Alt : Doc → Doc → Doc
}

export Doc, Text, Line, Concat, Group

fn doc_content (doc : Doc) : List Char =
  match doc {
    Text chars ↦ chars;
    Line ↦ Nil Char;
    Concat left right ↦ list_append Char (doc_content left) (doc_content right);
    Nest amount body ↦ doc_content body;
    Group body ↦ doc_content body;
    Alt first second ↦ doc_content first
  }

fn DocContentInvariant (doc : Doc) : Prop =
  match doc {
    Text chars ↦ Top;
    Line ↦ Top;
    Concat left right ↦ And (DocContentInvariant left) (DocContentInvariant right);
    Nest amount body ↦ DocContentInvariant body;
    Group body ↦ DocContentInvariant body;
    Alt first second ↦
      And
        (DocContentInvariant first)
        (And
          (DocContentInvariant second)
          (Equal (List Char) (doc_content first) (doc_content second)))
  }
```

`DocContentInvariant` is structural except at `Alt`: its two branches must
carry the same text tokens. This is the condition under which choosing a layout
can change whitespace without changing what the document says.

## 2. Fitting and rendering

The fitting rule is exact: the flat width of a document counts every text
character and counts each `Line` as one column; nesting changes indentation but
not flat width. A layout fits when `flat_width ≤ width`, including equality.
`Group` uses its flat layout exactly when it fits. `Alt` uses its first layout
flat exactly when that layout fits, otherwise its second layout broken.

```ken
fn pretty_repeat_char (char : Char) (count : Nat) : List Char =
  match count {
    Zero ↦ Nil Char;
    Suc rest ↦ Cons Char char (pretty_repeat_char char rest)
  }

fn doc_flat_width (doc : Doc) : Nat =
  match doc {
    Text chars ↦ length Char chars;
    Line ↦ Suc Zero;
    Concat left right ↦ add (doc_flat_width left) (doc_flat_width right);
    Nest amount body ↦ doc_flat_width body;
    Group body ↦ doc_flat_width body;
    Alt first second ↦ doc_flat_width first
  }

fn doc_fits (width : Nat) (doc : Doc) : Bool = leq_nat (doc_flat_width doc) width

fn render_mode (flat : Bool) (width : Nat) (indent : Nat) (doc : Doc) : List Char =
  match doc {
    Text chars ↦ chars;
    Line ↦
      match flat {
        True ↦ Cons Char (32 : Int) (Nil Char);
        False ↦ Cons Char (10 : Int) (pretty_repeat_char (32 : Int) indent)
      };
    Concat left right ↦
      list_append
        Char
        (render_mode flat width indent left)
        (render_mode flat width indent right);
    Nest amount body ↦ render_mode flat width (add indent amount) body;
    Group body ↦
      match doc_fits width body {
        True ↦ render_mode True width indent body;
        False ↦ render_mode False width indent body
      };
    Alt first second ↦
      match doc_fits width first {
        True ↦ render_mode True width indent first;
        False ↦ render_mode False width indent second
      }
  }

pub fn render (width : Nat) (doc : Doc) : List Char = render_mode False width Zero doc
```

A fitting document remains flat even when a nested `Group` or `Alt` makes its
own choice. The addition bounds pass the fit premise to both `Concat` children;
length of list append and congruence reassemble their exact flat widths.

```ken
theorem pretty_add_cong
      (x : Nat)
      (x2 : Nat)
      (y : Nat)
      (y2 : Nat)
      (same_x : Equal Nat x x2)
      (same_y : Equal Nat y y2)
    : Equal Nat (add x y) (add x2 y2) =
  trans
    Nat
    (add x y)
    (add x2 y)
    (add x2 y2)
    (cong Nat Nat x x2 (λn. add n y) same_x)
    (cong Nat Nat y y2 (add x2) same_y)

theorem render_flat_fits
      (width : Nat) (indent : Nat) (doc : Doc)
    : Equal Bool (doc_fits width doc) True
      → Equal Nat (length Char (render_mode True width indent doc)) (doc_flat_width doc) =
  match doc {
    Text chars ↦ λfit. Refl;
    Line ↦ λfit. Proved;
    Concat left right ↦
      λfit.
        let
          left_width = doc_flat_width left;
          right_width = doc_flat_width right;
          joined_width = add left_width right_width;
          left_chars = render_mode True width indent left;
          right_chars = render_mode True width indent right;
          left_fit : Equal Bool (doc_fits width left) True =
            (proof trans for leq_nat)
              left_width
              joined_width
              width
              (leq_nat_add_left_bound left_width right_width)
              fit;
          right_fit : Equal Bool (doc_fits width right) True =
            (proof trans for leq_nat)
              right_width
              joined_width
              width
              (leq_nat_add_right_bound left_width right_width)
              fit
        in
          trans
            Nat
            (length Char (list_append Char left_chars right_chars))
            (add (length Char left_chars) (length Char right_chars))
            joined_width
            (list_append::length Char left_chars right_chars)
            (pretty_add_cong
              (length Char left_chars)
              left_width
              (length Char right_chars)
              right_width
              (render_flat_fits width indent left left_fit)
              (render_flat_fits width indent right right_fit));
    Nest amount body ↦ λfit. render_flat_fits width (add indent amount) body fit;
    Group body ↦
      λfit.
        J
          (λchoice _.
            Equal
              Nat
              (length
                Char
                (match choice {
                  True ↦ render_mode True width indent body;
                  False ↦ render_mode False width indent body
                }))
              (doc_flat_width body))
          (render_flat_fits width indent body fit)
          (sym Bool (doc_fits width body) True fit);
    Alt first second ↦
      λfit.
        J
          (λchoice _.
            Equal
              Nat
              (length
                Char
                (match choice {
                  True ↦ render_mode True width indent first;
                  False ↦ render_mode False width indent second
                }))
              (doc_flat_width first))
          (render_flat_fits width indent first fit)
          (sym Bool (doc_fits width first) True fit)
  }
```

The renderer has no ambient inputs: all choices are functions of `width` and
the `Doc` value. Broken `Line` emits newline followed by the current nesting
indent; flat `Line` emits one space.

## 3. Content laws

`render_content_mode` is the text-token projection of the same decisions made
by `render_mode`: it follows the same `Group`/`Alt` branches and ignores only
the layout separators introduced by `Line` and `Nest`. This gives the phrase
“rendered output's text content” a checked, unambiguous structural meaning even
when a text leaf itself contains whitespace.

```ken
fn render_content_mode (flat : Bool) (width : Nat) (doc : Doc) : List Char =
  match doc {
    Text chars ↦ chars;
    Line ↦ Nil Char;
    Concat left right ↦
      list_append
        Char
        (render_content_mode flat width left)
        (render_content_mode flat width right);
    Nest amount body ↦ render_content_mode flat width body;
    Group body ↦
      match doc_fits width body {
        True ↦ render_content_mode True width body;
        False ↦ render_content_mode False width body
      };
    Alt first second ↦
      match doc_fits width first {
        True ↦ render_content_mode True width first;
        False ↦ render_content_mode False width second
      }
  }

fn render_content (width : Nat) (doc : Doc) : List Char = render_content_mode False width doc

theorem pretty_list_append_cong
      (left : List Char)
      (left2 : List Char)
      (right : List Char)
      (right2 : List Char)
      (same_left : Equal (List Char) left left2)
      (same_right : Equal (List Char) right right2)
    : Equal (List Char) (list_append Char left right) (list_append Char left2 right2) =
  trans
    (List Char)
    (list_append Char left right)
    (list_append Char left2 right)
    (list_append Char left2 right2)
    (cong (List Char) (List Char) left left2 (λchars. list_append Char chars right) same_left)
    (cong (List Char) (List Char) right right2 (list_append Char left2) same_right)

fn pretty_bool_cases (b : Bool) : Or (Equal Bool b True) (Equal Bool b False) =
  match b {
    True ↦ Inl (Equal Bool True True) (Equal Bool True False) Proved;
    False ↦ Inr (Equal Bool False True) (Equal Bool False False) Proved
  }

theorem render_group_when_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (body : Doc)
      (fit : Equal Bool (doc_fits width body) True)
    : Equal
        (List Char)
        (render_mode flat width indent (Group body))
        (render_mode True width indent body) =
  match pretty_bool_cases (doc_fits width body) {
    Inl h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_mode True width indent body;
              False ↦ render_mode False width indent body
            })
            (render_mode True width indent body))
        Refl
        (sym Bool (doc_fits width body) True h);
    Inr h ↦
      absurd
        (trans
          Bool
          True
          (doc_fits width body)
          False
          (sym Bool (doc_fits width body) True fit)
          h)
  }

theorem render_group_when_not_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (body : Doc)
      (no_fit : Equal Bool (doc_fits width body) False)
    : Equal
        (List Char)
        (render_mode flat width indent (Group body))
        (render_mode False width indent body) =
  match pretty_bool_cases (doc_fits width body) {
    Inl h ↦
      absurd
        (trans
          Bool
          False
          (doc_fits width body)
          True
          (sym Bool (doc_fits width body) False no_fit)
          h);
    Inr h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_mode True width indent body;
              False ↦ render_mode False width indent body
            })
            (render_mode False width indent body))
        Refl
        (sym Bool (doc_fits width body) False h)
  }

theorem render_alt_when_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (first : Doc)
      (second : Doc)
      (fit : Equal Bool (doc_fits width first) True)
    : Equal
        (List Char)
        (render_mode flat width indent (Alt first second))
        (render_mode True width indent first) =
  match pretty_bool_cases (doc_fits width first) {
    Inl h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_mode True width indent first;
              False ↦ render_mode False width indent second
            })
            (render_mode True width indent first))
        Refl
        (sym Bool (doc_fits width first) True h);
    Inr h ↦
      absurd
        (trans
          Bool
          True
          (doc_fits width first)
          False
          (sym Bool (doc_fits width first) True fit)
          h)
  }

theorem render_alt_when_not_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (first : Doc)
      (second : Doc)
      (no_fit : Equal Bool (doc_fits width first) False)
    : Equal
        (List Char)
        (render_mode flat width indent (Alt first second))
        (render_mode False width indent second) =
  match pretty_bool_cases (doc_fits width first) {
    Inl h ↦
      absurd
        (trans
          Bool
          False
          (doc_fits width first)
          True
          (sym Bool (doc_fits width first) False no_fit)
          h);
    Inr h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_mode True width indent first;
              False ↦ render_mode False width indent second
            })
            (render_mode False width indent second))
        Refl
        (sym Bool (doc_fits width first) False h)
  }

theorem render_content_group_preserves
      (flat : Bool) (width : Nat) (body : Doc) (valid : DocContentInvariant body)
    : Equal
        (List Char)
        (render_content_mode flat width (Group body))
        (doc_content (Group body)) =
  match pretty_bool_cases (doc_fits width body) {
    Inl h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_content_mode True width body;
              False ↦ render_content_mode False width body
            })
            (doc_content body))
        (render_content_mode_preserves True width body valid)
        (sym Bool (doc_fits width body) True h);
    Inr h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_content_mode True width body;
              False ↦ render_content_mode False width body
            })
            (doc_content body))
        (render_content_mode_preserves False width body valid)
        (sym Bool (doc_fits width body) False h)
  }

theorem render_content_alt_preserves
      (flat : Bool)
      (width : Nat)
      (first : Doc)
      (second : Doc)
      (first_valid : DocContentInvariant first)
      (second_valid : DocContentInvariant second)
      (same_content : Equal (List Char) (doc_content first) (doc_content second))
    : Equal
        (List Char)
        (render_content_mode flat width (Alt first second))
        (doc_content (Alt first second)) =
  match pretty_bool_cases (doc_fits width first) {
    Inl h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_content_mode True width first;
              False ↦ render_content_mode False width second
            })
            (doc_content first))
        (render_content_mode_preserves True width first first_valid)
        (sym Bool (doc_fits width first) True h);
    Inr h ↦
      J
        (λchoice _.
          Equal
            (List Char)
            (match choice {
              True ↦ render_content_mode True width first;
              False ↦ render_content_mode False width second
            })
            (doc_content first))
        (trans
          (List Char)
          (render_content_mode False width second)
          (doc_content second)
          (doc_content first)
          (render_content_mode_preserves False width second second_valid)
          (sym (List Char) (doc_content first) (doc_content second) same_content))
        (sym Bool (doc_fits width first) False h)
  }

theorem render_content_mode_preserves
      (flat : Bool) (width : Nat) (doc : Doc)
    : (valid : DocContentInvariant doc)
      → Equal (List Char) (render_content_mode flat width doc) (doc_content doc) =
  match doc {
    Text chars ↦ λvalid. Refl;
    Line ↦ λvalid. Proved;
    Concat left right ↦
      λvalid.
        pretty_list_append_cong
          (render_content_mode flat width left)
          (doc_content left)
          (render_content_mode flat width right)
          (doc_content right)
          (render_content_mode_preserves
            flat
            width
            left
            (and_fst (DocContentInvariant left) (DocContentInvariant right) valid))
          (render_content_mode_preserves
            flat
            width
            right
            (and_snd (DocContentInvariant left) (DocContentInvariant right) valid));
    Nest amount body ↦ λvalid. render_content_mode_preserves flat width body valid;
    Group body ↦ λvalid. render_content_group_preserves flat width body valid;
    Alt first second ↦
      λvalid.
        render_content_alt_preserves
          flat
          width
          first
          second
          (and_fst
            (DocContentInvariant first)
            (And
              (DocContentInvariant second)
              (Equal (List Char) (doc_content first) (doc_content second)))
            valid)
          (and_fst
            (DocContentInvariant second)
            (Equal (List Char) (doc_content first) (doc_content second))
            (and_snd
              (DocContentInvariant first)
              (And
                (DocContentInvariant second)
                (Equal (List Char) (doc_content first) (doc_content second)))
              valid))
          (and_snd
            (DocContentInvariant second)
            (Equal (List Char) (doc_content first) (doc_content second))
            (and_snd
              (DocContentInvariant first)
              (And
                (DocContentInvariant second)
                (Equal (List Char) (doc_content first) (doc_content second)))
              valid))
  }

proof preserves_text_tokens for render_content_mode
      (flat : Bool) (width : Nat) (doc : Doc) (valid : DocContentInvariant doc)
    : Equal (List Char) (render_content_mode flat width doc) (doc_content doc) =
  render_content_mode_preserves flat width doc valid

proof preserves_text_tokens for render_content
      (width : Nat) (doc : Doc) (valid : DocContentInvariant doc)
    : Equal (List Char) (render_content width doc) (doc_content doc) =
  (proof preserves_text_tokens for render_content_mode) False width doc valid

proof width_independent for render_content
      (first_width : Nat) (second_width : Nat) (doc : Doc) (valid : DocContentInvariant doc)
    : Equal (List Char) (render_content first_width doc) (render_content second_width doc) =
  trans
    (List Char)
    (render_content first_width doc)
    (doc_content doc)
    (render_content second_width doc)
    ((proof preserves_text_tokens for render_content) first_width doc valid)
    (sym
      (List Char)
      (render_content second_width doc)
      (doc_content doc)
      ((proof preserves_text_tokens for render_content) second_width doc valid))

proof fixed_point for render
      (width : Nat) (doc : Doc)
    : Equal (List Char) (render width (Text (render width doc))) (render width doc) =
  Refl
```

The fixed-point statement is exact: once a rendered `List Char` is embedded as
one inert `Text` leaf, rendering it again at the same width returns the same
characters byte-for-byte. It adds no new layout decisions. The four choice
laws above quantify over the incoming flat mode: `Group` and `Alt` ignore it,
then select from their own fit computation. A false fit premise selects the
broken branch; no content invariant is required for a layout equality.

These generic consumers keep each fitting and choice statement usable at its
full parameter and premise types, rather than merely at a closed document.

```ken example
const doc_fitting_counterexample : Doc =
  Group (Concat (Text (Cons Char (97 : Int) (Nil Char))) Line)

theorem doc_example_counterexample_does_not_fit
    : Equal Bool (doc_fits Zero doc_fitting_counterexample) False =
  Proved

theorem doc_example_counterexample_broken_length
    : Equal Nat
        (length Char (render_mode True Zero (Suc (Suc Zero)) doc_fitting_counterexample))
        (Suc (Suc (Suc (Suc Zero)))) =
  Proved

theorem doc_example_counterexample_flat_width
    : Equal Nat (doc_flat_width doc_fitting_counterexample) (Suc (Suc Zero)) =
  Proved
```

```ken example
theorem doc_example_flat_fits
      (width : Nat) (indent : Nat) (doc : Doc) (fit : Equal Bool (doc_fits width doc) True)
    : Equal Nat (length Char (render_mode True width indent doc)) (doc_flat_width doc) =
  render_flat_fits width indent doc fit
```

```ken example
theorem doc_example_group_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (body : Doc)
      (fit : Equal Bool (doc_fits width body) True)
    : Equal
        (List Char)
        (render_mode flat width indent (Group body))
        (render_mode True width indent body) =
  render_group_when_fits flat width indent body fit
```

```ken example
theorem doc_example_group_not_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (body : Doc)
      (no_fit : Equal Bool (doc_fits width body) False)
    : Equal
        (List Char)
        (render_mode flat width indent (Group body))
        (render_mode False width indent body) =
  render_group_when_not_fits flat width indent body no_fit
```

```ken example
theorem doc_example_alt_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (first : Doc)
      (second : Doc)
      (fit : Equal Bool (doc_fits width first) True)
    : Equal
        (List Char)
        (render_mode flat width indent (Alt first second))
        (render_mode True width indent first) =
  render_alt_when_fits flat width indent first second fit
```

```ken example
theorem doc_example_alt_not_fits
      (flat : Bool)
      (width : Nat)
      (indent : Nat)
      (first : Doc)
      (second : Doc)
      (no_fit : Equal Bool (doc_fits width first) False)
    : Equal
        (List Char)
        (render_mode flat width indent (Alt first second))
        (render_mode False width indent second) =
  render_alt_when_not_fits flat width indent first second no_fit
```

The fit premise cannot be dropped. At width zero the one-character `Text`
followed by `Line` in a `Group` chooses the broken rendering, regardless of
its incoming flat mode. At indent two that rendering contains four characters,
while its flat width is two.

```ken reject
const doc_reject_fitting_counterexample : Doc =
  Group (Concat (Text (Cons Char (97 : Int) (Nil Char))) Line)

theorem doc_reject_flat_without_fit
    : Equal Nat
        (length Char (render_mode True Zero (Suc (Suc Zero)) doc_reject_fitting_counterexample))
        (doc_flat_width doc_reject_fitting_counterexample) =
  Refl
```

## 4. String boundary

String conversion is deliberately thin and proof-free. Every verified law
above remains at the structural `List Char` layer.

```ken
pub fn text_string (value : String) : Doc = Text (string_to_list_char value)

fn render_string (width : Nat) (doc : Doc) : String = list_char_to_string (render width doc)

theorem render_string_view
      (width : Nat)
      (doc : Doc)
      (round_trip : Equal
        (List Char)
        (string_to_list_char (list_char_to_string (render width doc)))
        (render width doc))
    : Equal (List Char) (string_to_list_char (render_string width doc)) (render width doc) =
  round_trip
```

The list view of `render_string` agrees with `render` only when encoding and
then decoding this specific rendered list preserves it. In particular, no
unconditional String round trip or normalization-free conversion is claimed.

```ken example
theorem doc_example_render_string_view
      (width : Nat)
      (doc : Doc)
      (round_trip : Equal
        (List Char)
        (string_to_list_char (list_char_to_string (render width doc)))
        (render width doc))
    : Equal (List Char) (string_to_list_char (render_string width doc)) (render width doc) =
  render_string_view width doc round_trip
```

## 5. Trust and derivation

Every declaration is transparent ordinary Ken. Recursion is structural on
`Nat`, `List`, or `Doc`; equality reasoning uses the landed `Transport`
combinators. The package adds no primitive, postulate, `Axiom`, or trusted-base
entry, and it has no dependency on `Diagnostic`. Its flat-width proof uses
`Data.Numeric.Nat.Order`'s checked addition bounds and Derived's checked
append-length law. Its four private choice laws follow the renderer's own
Boolean branch and require no content invariant. The String-view law carries
the rendered list's explicit round-trip premise: it supplies no unconditional
claim about the opaque conversion.
