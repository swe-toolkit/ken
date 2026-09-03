# `Data.Text.StringKeys` — lawful String equality and order

Canonical String equality now lives with `DecEq` in
`Core.Classes.LawfulClasses`. This package retains the transported `Ord String`
dictionary and imports every non-floor dependency explicitly.

## 1. Transported operations and laws

```ken
import Core.Classes.LawfulClasses (IsTrue, Ord, bool_or)

import Core.Logic.Transport (cong)

import Data.Text.StringBijection (string_to_list_char_injective)

fn string_ord_leq (left : String) (right : String) : Bool =
  (Ord_instance_List Char Ord_instance_Char).leq
    (string_to_list_char left)
    (string_to_list_char right)

proof refl for string_ord_leq (text : String) : IsTrue (string_ord_leq text text) =
  (Ord_instance_List Char Ord_instance_Char).refl (string_to_list_char text)

proof antisym for string_ord_leq
      (left : String)
      (right : String)
      (forward : IsTrue (string_ord_leq left right))
      (reverse : IsTrue (string_ord_leq right left))
    : Equal String left right =
  string_to_list_char_injective
    left
    right
    ((Ord_instance_List Char Ord_instance_Char).antisym
      (string_to_list_char left)
      (string_to_list_char right)
      forward
      reverse)

proof trans for string_ord_leq
      (left : String)
      (middle : String)
      (right : String)
      (first : IsTrue (string_ord_leq left middle))
      (second : IsTrue (string_ord_leq middle right))
    : IsTrue (string_ord_leq left right) =
  (Ord_instance_List Char Ord_instance_Char).trans
    (string_to_list_char left)
    (string_to_list_char middle)
    (string_to_list_char right)
    first
    second

proof total for string_ord_leq
      (left : String) (right : String)
    : IsTrue (bool_or (string_ord_leq left right) (string_ord_leq right left)) =
  (Ord_instance_List Char Ord_instance_Char).total
    (string_to_list_char left)
    (string_to_list_char right)

instance Ord String {
  leq = string_ord_leq;
  refl = proof refl for string_ord_leq;
  antisym = proof antisym for string_ord_leq;
  trans = proof trans for string_ord_leq;
  total = proof total for string_ord_leq
}
```

## 2. Checked examples

The transported operations compute through `string_to_list_char` and the
landed structural dictionaries.

```ken example
const string_key_equal_example : Bool = string_deceq_eq "alpha" "alpha"

const string_key_distinct_example : Bool = string_deceq_eq "alpha" "beta"

const string_key_order_example : Bool = string_ord_leq "alpha" "beta"
```

## 3. Trust and derivation

This package contains no `Axiom`. Its `Ord` dictionary is a transparent transport through the existing lawful
`List Char` dictionary. `antisym` cites the prerequisite injectivity certificate;
`refl`, `trans`, and `total` use only dictionary projections. Bytes keys remain
outside this package.
