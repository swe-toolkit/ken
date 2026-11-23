# Splitting an offset bounded search

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.8.

This seed exercises the two public offset-split laws on the measured
closed fixtures. The shift premise is checked only below `width`.
The accepted Some and None applications each have direct result
controls. The refusal changes the first-part result premise from true
to false, with a separate false-conclusion control. These finite
applications do not establish either generic proof body. The seed has
not been run by the official conformance runner. No CI, publication,
landing, or trust measurement is claimed.

## Shared fixture and accepted applications

`hits` is true exactly at five and seven. `after_three` shifts that
predicate by three; `after_six` is used in the refusal below. Closed
constructor equalities use `Proved`, while the open shift equations
use `Refl`, matching the measured forms.

```ken
import Algorithm.Optimization.BoundedSearch
  (find_below, find_below_offset, find_below_offset_none)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (lt_nat)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const seven : Nat = Suc six

const eight : Nat = Suc seven

fn hits (n : Nat) : Bool =
  match n {
    Zero ↦ False;
    Suc n1 ↦
      match n1 {
        Zero ↦ False;
        Suc n2 ↦
          match n2 {
            Zero ↦ False;
            Suc n3 ↦
              match n3 {
                Zero ↦ False;
                Suc n4 ↦
                  match n4 {
                    Zero ↦ False;
                    Suc n5 ↦
                      match n5 {
                        Zero ↦ True;
                        Suc n6 ↦
                          match n6 {
                            Zero ↦ False;
                            Suc n7 ↦
                              match n7 {
                                Zero ↦ True;
                                Suc n8 ↦ False
                              }
                          }
                      }
                  }
              }
          }
      }
  }

fn after_three (m : Nat) : Bool = hits (add three m)

fn after_six (m : Nat) : Bool = hits (add six m)

theorem shift_three_below_four
    : (m : Nat) → Equal Bool (lt_nat m four) True
      → Equal Bool (after_three m) (hits (add three m)) =
  λm. λwithin. Refl

theorem shift_three_below_two
    : (m : Nat) → Equal Bool (lt_nat m two) True
      → Equal Bool (after_three m) (hits (add three m)) =
  λm. λwithin. Refl

theorem first_part_three_is_none
    : Equal (Option Nat) (find_below hits three) (None Nat) =
  Proved

theorem shifted_part_four_finds_two
    : Equal (Option Nat) (find_below after_three four) (Some Nat two) =
  Proved

theorem offset_three_finds_five_by_law
    : Equal (Option Nat) (find_below hits seven) (Some Nat five) =
      find_below_offset
        hits
        after_three
        three
        four
        two
        shift_three_below_four
        first_part_three_is_none
        shifted_part_four_finds_two

theorem offset_three_finds_five_by_proved
    : Equal (Option Nat) (find_below hits seven) (Some Nat five) =
  Proved

theorem shifted_part_two_is_none
    : Equal (Option Nat) (find_below after_three two) (None Nat) =
  Proved

theorem offset_three_none_by_law
    : Equal (Option Nat) (find_below hits five) (None Nat) =
      find_below_offset_none
        hits
        after_three
        three
        two
        shift_three_below_two
        first_part_three_is_none
        shifted_part_two_is_none

theorem offset_three_none_by_proved
    : Equal (Option Nat) (find_below hits five) (None Nat) =
  Proved
```

### `stdlib/optimization/bounded-search-offset-split-some`

- spec: §2.8.
- promise class: durable invariant.
- given: `hits`, true exactly at five and seven; `after_three`; offset
  three, width four; the shift below four; `find_below hits three =
  None`; and `find_below after_three four = Some two`.
- expect: `find_below_offset` accepts and concludes
  `find_below hits seven = Some five`.
- measured: the shift is supplied only for `m < four`; the first-part
  miss and shifted-part result are the public law's next premises.
  A direct `Proved` result control checks the same conclusion.
- claimed: the public Some law transports the shifted least hit to the
  corresponding offset result when the first part has no hit.
- gap: these closed inputs do not prove the universally quantified law.
- why: five is the first hit at or beyond offset three; seven is a
  later hit and does not replace it.

### `stdlib/optimization/bounded-search-offset-split-none`

- spec: §2.8.
- promise class: durable invariant.
- given: the same predicate and offset, width two; the shift below
  two; `find_below hits three = None`; and
  `find_below after_three two = None`.
- expect: `find_below_offset_none` accepts and concludes
  `find_below hits five = None`.
- measured: both subranges have direct `Proved` result controls, and
  another direct `Proved` control checks the split conclusion.
- claimed: the public None companion transports an empty shifted
  subrange when the initial subrange is also empty.
- gap: this finite instance does not prove the generic companion law.
- why: the companion requires both parts to be refuted; it does not
  assert stability of `None` under every increase of the bound.

## Refusal at a false first-part premise

For this row, use a fresh accepted prefix in the same roots-loaded
environment, then append the declarations below. `after_six` shifts
`hits` by six. Its search below two finds one, and the whole search
below eight independently returns five. The attempted None premise is
false because `hits` already has its first hit at five. The law
application is present after that premise; the expected refusal is at
the premise declaration, with D0-measured
`KernelRejected(TypeMismatch)`. D0 observed span 1..91 for that
premise in its fixture; this seed does not promise an absolute span.

### `stdlib/optimization/bounded-search-offset-split-false-none-premise`

- spec: §2.8.
- promise class: durable invariant.
- given: the accepted prefix, then `after_six`, its shift below two,
  and `find_below after_six two = Some one`; also the direct controls
  `find_below hits six = Some five` and `find_below hits eight =
  Some five`.
- expect: the false declaration
  `find_below hits six = None` refuses with
  `KernelRejected(TypeMismatch)`; do not count a refusal elsewhere in
  the prefix as success.
- measured: the proposed first-part miss is contradicted by the
  same-bound `Some five` control, the whole search below eight also
  returns `Some five`, and the shifted subrange has result `Some one`.
- claimed: the public law's first-part `None` premise is required; the
  law must not infer an offset result when the prefix already contains
  a hit.
- gap: this finite refusal does not establish the generic law or a
  more specific diagnostic span or wording.
- why: the exact premise is false for `hits` at offset six.

```ken
-- Append to a fresh accepted prefix from the preceding block.
theorem shift_six_below_two
    : (m : Nat) → Equal Bool (lt_nat m two) True
      → Equal Bool (after_six m) (hits (add six m)) =
  λm. λwithin. Refl

theorem shifted_part_six_finds_one
    : Equal (Option Nat) (find_below after_six two) (Some Nat one) =
  Proved

theorem hits_below_six_finds_five
    : Equal (Option Nat) (find_below hits six) (Some Nat five) =
  Proved

theorem hits_below_eight_finds_five
    : Equal (Option Nat) (find_below hits eight) (Some Nat five) =
  Proved

theorem first_part_six_is_none
    : Equal (Option Nat) (find_below hits six) (None Nat) =
  Proved

theorem false_premise_offset_result
    : Equal (Option Nat) (find_below hits eight) (Some Nat seven) =
      find_below_offset
        hits
        after_six
        six
        two
        one
        shift_six_below_two
        first_part_six_is_none
        shifted_part_six_finds_one
```

## Refusal of a false Some conclusion

Use a fresh accepted prefix, then append the two result claims below.
The true `Some five` control has the same proposition and proof form as
its false `Some seven` neighbor except for the claimed result. Require
that the former checks and the latter refuses with
`KernelRejected(TypeMismatch)` at its declaration.

### `stdlib/optimization/bounded-search-offset-split-false-some-result`

- spec: §2.1 and §2.8.
- promise class: durable invariant.
- given: the accepted prefix, then the `Some five` control and the
  `Some seven` claim for `find_below hits eight`, each proved by
  `Proved`.
- expect: the `Some five` control checks; the false `Some seven`
  claim refuses with `KernelRejected(TypeMismatch)`.
- measured: `find_below hits eight` returns `Some five`; the same-form
  positive control checks while the distinct claimed result refuses.
- claimed: the split law does not justify a false result when its
  first-part miss premise is false.
- gap: these closed results do not prove the public law or its generic
  proof body.
- why: a bare false `Some seven` assertion must fail even without the
  split-law premise; `Some five` is the discriminating value control.

```ken
-- Append to a fresh accepted prefix from the preceding block.
theorem hits_below_eight_is_some_five
    : Equal (Option Nat) (find_below hits eight) (Some Nat five) =
  Proved

theorem reject_some_seven_by_proved
    : Equal (Option Nat) (find_below hits eight) (Some Nat seven) =
  Proved
```

## Coverage boundary

The seed covers two checked applications of the public laws, their
measured premises and direct value controls, a refusal at the false
first-part premise, and a separate same-form false-result refusal. It
does not claim generic proof-body coverage, runtime strategy,
predicate-call counts, performance, official conformance execution,
CI, publication, landing, or trust measurement.
