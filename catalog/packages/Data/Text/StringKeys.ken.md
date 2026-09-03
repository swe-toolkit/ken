# `Data.Text.StringKeys` — lawful String equality and order

Canonical String equality and order live with their classes in
`Core.Classes.LawfulClasses`. This compatibility package imports the class owner
and the operations used by its examples.

## 1. Canonical providers

```ken
import Core.Classes.LawfulClasses (DecEq, Ord, string_deceq_eq, string_ord_leq)
```

## 2. Checked examples

The imported operations compute through `string_to_list_char` and the canonical
structural dictionaries.

```ken example
const string_key_equal_example : Bool = string_deceq_eq "alpha" "alpha"

const string_key_distinct_example : Bool = string_deceq_eq "alpha" "beta"

const string_key_order_example : Bool = string_ord_leq "alpha" "beta"
```

## 3. Trust and derivation

This package declares no local trust and no local class instance. The imported
class owner retains the equality and order families with their existing trust
accounting. Bytes keys remain outside this package.
