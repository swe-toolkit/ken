# Capability.Process.Arguments

`Capability.Process.Arguments` is the pure, byte-preserving view of the argv
field in the landed `ProcessInput` ABI. Raw arguments remain `Bytes`; decoding
is always an explicit choice made by a caller.

## 1. Raw process input

The runner's third `ProcessInput` field is the current working directory. This
package names only the argv projection and replacement operation, while the
match keeps the environment and working-directory bytes unchanged.

```ken
import Capability.Parsing.Cursor (ArgLocation, MkArgLocation)

import Core.Classes.LawfulClasses (leq_nat)

import Data.Collections.Derived (bytes_nat_length, nth)

pub fn process_arguments (input : ProcessInput) : List Bytes =
  match input {
    MkProcessInput arguments environment working_directory ↦ arguments
  }

pub fn replace_process_arguments
      (arguments : List Bytes) (input : ProcessInput)
    : ProcessInput =
  match input {
    MkProcessInput previous environment working_directory ↦
      MkProcessInput arguments environment working_directory
  }

pub proof round_trip for process_arguments
      (arguments : List Bytes) (input : ProcessInput)
    : Equal
        (List Bytes)
        (process_arguments (replace_process_arguments arguments input))
        arguments =
  match input {
    MkProcessInput previous environment working_directory ↦ Refl
  }

pub fn process_argument_at (index : Nat) (input : ProcessInput) : Option Bytes =
  nth Bytes index (process_arguments input)
```

## 2. Arguments and locations

Parsing consumes raw argument `Bytes`. Positional lookup exposes those bytes
directly, and bounds are checked against their structural `Nat` length through
the canonical `leq_nat` relation.

`argument_slice_location` accepts only a range whose argument exists, whose
start does not exceed its end, and whose end does not exceed the computed byte
length. The resulting location uses the shared `ArgLocation` carrier.

```ken
pub fn argument_at (index : Nat) (arguments : List Bytes) : Option Bytes =
  nth Bytes index arguments

fn argument_bytes_at (index : Nat) (arguments : List Bytes) : Option Bytes =
  match argument_at index arguments {
    None ↦ None Bytes;
    Some argument ↦ Some Bytes argument
  }

pub fn argument_slice_location
      (index : Nat) (start : Nat) (end : Nat) (arguments : List Bytes)
    : Option ArgLocation =
  match argument_at index arguments {
    None ↦ None ArgLocation;
    Some argument ↦
      match leq_nat start end {
        False ↦ None ArgLocation;
        True ↦
          match leq_nat end (bytes_nat_length argument) {
            False ↦ None ArgLocation;
            True ↦ Some ArgLocation (MkArgLocation index start end)
          }
      }
  }

pub proof soundness for argument_slice_location
      (index : Nat)
      (start : Nat)
      (end : Nat)
      (arguments : List Bytes)
      (argument : Bytes)
      (hargument : Equal (Option Bytes) (argument_at index arguments) (Some Bytes argument))
      (hordered : Equal Bool (leq_nat start end) True)
      (hbounded : Equal Bool (leq_nat end (bytes_nat_length argument)) True)
    : Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (Some ArgLocation (MkArgLocation index start end)) =
  J
    (λlookup _.
      Equal
        (Option ArgLocation)
        (match lookup {
          None ↦ None ArgLocation;
          Some found ↦
            match leq_nat start end {
              False ↦ None ArgLocation;
              True ↦
                match leq_nat end (bytes_nat_length found) {
                  False ↦ None ArgLocation;
                  True ↦ Some ArgLocation (MkArgLocation index start end)
                }
            }
        })
        (Some ArgLocation (MkArgLocation index start end)))
    (J
      (λordered _.
        Equal
          (Option ArgLocation)
          (match ordered {
            False ↦ None ArgLocation;
            True ↦
              match leq_nat end (bytes_nat_length argument) {
                False ↦ None ArgLocation;
                True ↦ Some ArgLocation (MkArgLocation index start end)
              }
          })
          (Some ArgLocation (MkArgLocation index start end)))
      (J
        (λbounded _.
          Equal
            (Option ArgLocation)
            (match bounded {
              False ↦ None ArgLocation;
              True ↦ Some ArgLocation (MkArgLocation index start end)
            })
            (Some ArgLocation (MkArgLocation index start end)))
        (and_intro
          (Equal Nat index index)
          (And (Equal Nat start start) (Equal Nat end end))
          Refl
          (and_intro (Equal Nat start start) (Equal Nat end end) Refl Refl))
        (J
          (λright _. Equal Bool right (leq_nat end (bytes_nat_length argument)))
          Refl
          hbounded))
      (J (λright _. Equal Bool right (leq_nat start end)) Refl hordered))
    (J (λright _. Equal (Option Bytes) right (argument_at index arguments)) Refl hargument)

theorem successful_argument_from_lookup
      (index : Nat)
      (start : Nat)
      (end : Nat)
      (arguments : List Bytes)
      (loc : ArgLocation)
      (search_index : Nat)
      (search_arguments : List Bytes)
    : Equal
        (Option Bytes)
        (argument_at index arguments)
        (argument_at search_index search_arguments)
      → Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (Some ArgLocation loc)
      → (goal : Prop)
      → ((argument : Bytes)
          → Equal
          (Option Bytes)
          (argument_at index arguments)
          (Some Bytes argument)
          → goal)
      → goal =
  match search_arguments {
    Nil ↦
      λsame.
        λh.
          λgoal.
            λrecover.
              absurd
                (J
                  (λlookup _.
                    Equal
                      (Option ArgLocation)
                      (match lookup {
                        None ↦ None ArgLocation;
                        Some argument ↦
                          match leq_nat start end {
                            False ↦ None ArgLocation;
                            True ↦
                              match leq_nat end (bytes_nat_length argument) {
                                False ↦ None ArgLocation;
                                True ↦ Some ArgLocation (MkArgLocation index start end)
                              }
                          }
                      })
                      (Some ArgLocation loc))
                  h
                  same);
    Cons head tail ↦
      match search_index {
        Zero ↦ λsame. λh. λgoal. λrecover. recover head same;
        Suc search_index2 ↦
          λsame.
            λh.
              λgoal.
                λrecover.
                  successful_argument_from_lookup
                    index
                    start
                    end
                    arguments
                    loc
                    search_index2
                    tail
                    same
                    h
                    goal
                    recover
      }
  }

theorem argument_slice_location_complete_at
      (index : Nat)
      (start : Nat)
      (end : Nat)
      (arguments : List Bytes)
      (argument : Bytes)
      (loc : ArgLocation)
      (hargument : Equal (Option Bytes) (argument_at index arguments) (Some Bytes argument))
      (h : Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (Some ArgLocation loc))
    : (goal : Prop)
      → (Equal
          Bool
          (leq_nat start end)
          True
          → Equal
          Bool
          (leq_nat end (bytes_nat_length argument))
          True
          → Equal
          ArgLocation
          loc
          (MkArgLocation index start end)
          → goal)
      → goal =
  match leq_nat start end eqn : hordered {
    False ↦
      λgoal.
        λrecover.
          absurd
            (J
              (λordered _.
                Equal
                  (Option ArgLocation)
                  (match ordered {
                    False ↦ None ArgLocation;
                    True ↦
                      match leq_nat end (bytes_nat_length argument) {
                        False ↦ None ArgLocation;
                        True ↦ Some ArgLocation (MkArgLocation index start end)
                      }
                  })
                  (Some ArgLocation loc))
              (J
                (λlookup _.
                  Equal
                    (Option ArgLocation)
                    (match lookup {
                      None ↦ None ArgLocation;
                      Some found ↦
                        match leq_nat start end {
                          False ↦ None ArgLocation;
                          True ↦
                            match leq_nat end (bytes_nat_length found) {
                              False ↦ None ArgLocation;
                              True ↦ Some ArgLocation (MkArgLocation index start end)
                            }
                        }
                    })
                    (Some ArgLocation loc))
                h
                hargument)
              hordered);
    True ↦
      match leq_nat end (bytes_nat_length argument) eqn : hbounded {
        False ↦
          λgoal.
            λrecover.
              absurd
                (J
                  (λbounded _.
                    Equal
                      (Option ArgLocation)
                      (match bounded {
                        False ↦ None ArgLocation;
                        True ↦ Some ArgLocation (MkArgLocation index start end)
                      })
                      (Some ArgLocation loc))
                  (J
                    (λordered _.
                      Equal
                        (Option ArgLocation)
                        (match ordered {
                          False ↦ None ArgLocation;
                          True ↦
                            match leq_nat end (bytes_nat_length argument) {
                              False ↦ None ArgLocation;
                              True ↦ Some ArgLocation (MkArgLocation index start end)
                            }
                        })
                        (Some ArgLocation loc))
                    (J
                      (λlookup _.
                        Equal
                          (Option ArgLocation)
                          (match lookup {
                            None ↦ None ArgLocation;
                            Some found ↦
                              match leq_nat start end {
                                False ↦ None ArgLocation;
                                True ↦
                                  match leq_nat end (bytes_nat_length found) {
                                    False ↦ None ArgLocation;
                                    True ↦ Some ArgLocation (MkArgLocation index start end)
                                  }
                              }
                          })
                          (Some ArgLocation loc))
                      h
                      hargument)
                    hordered)
                  hbounded);
        True ↦
          λgoal.
            λrecover.
              recover
                Proved
                Proved
                (J
                  (λright _. Equal ArgLocation right (MkArgLocation index start end))
                  (and_intro
                    (Equal Nat index index)
                    (And (Equal Nat start start) (Equal Nat end end))
                    Refl
                    (and_intro (Equal Nat start start) (Equal Nat end end) Refl Refl))
                  (J
                    (λbounded _.
                      Equal
                        (Option ArgLocation)
                        (match bounded {
                          False ↦ None ArgLocation;
                          True ↦ Some ArgLocation (MkArgLocation index start end)
                        })
                        (Some ArgLocation loc))
                    (J
                      (λordered _.
                        Equal
                          (Option ArgLocation)
                          (match ordered {
                            False ↦ None ArgLocation;
                            True ↦
                              match leq_nat end (bytes_nat_length argument) {
                                False ↦ None ArgLocation;
                                True ↦ Some ArgLocation (MkArgLocation index start end)
                              }
                          })
                          (Some ArgLocation loc))
                      (J
                        (λlookup _.
                          Equal
                            (Option ArgLocation)
                            (match lookup {
                              None ↦ None ArgLocation;
                              Some found ↦
                                match leq_nat start end {
                                  False ↦ None ArgLocation;
                                  True ↦
                                    match leq_nat end (bytes_nat_length found) {
                                      False ↦ None ArgLocation;
                                      True ↦ Some ArgLocation (MkArgLocation index start end)
                                    }
                                }
                            })
                            (Some ArgLocation loc))
                        h
                        hargument)
                      hordered)
                    hbounded))
      }
  }

pub proof completeness for argument_slice_location
      (index : Nat) (start : Nat) (end : Nat) (arguments : List Bytes) (loc : ArgLocation)
    : Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (Some ArgLocation loc)
      → (goal : Prop)
      → ((argument : Bytes)
          → Equal
          (Option Bytes)
          (argument_at index arguments)
          (Some Bytes argument)
          → Equal
          Bool
          (leq_nat start end)
          True
          → Equal
          Bool
          (leq_nat end (bytes_nat_length argument))
          True
          → Equal
          ArgLocation
          loc
          (MkArgLocation index start end)
          → goal)
      → goal =
  λh.
    λgoal.
      λrecover.
        successful_argument_from_lookup
          index
          start
          end
          arguments
          loc
          index
          arguments
          Refl
          h
          goal
          (λargument.
            λhargument.
              argument_slice_location_complete_at
                index
                start
                end
                arguments
                argument
                loc
                hargument
                h
                goal
                (λhordered.
                  λhbounded.
                    λhlocation. recover argument hargument hordered hbounded hlocation))

pub proof refuses_missing_argument for argument_slice_location
      (index : Nat)
      (start : Nat)
      (end : Nat)
      (arguments : List Bytes)
      (argument : Bytes)
      (hmissing : Equal (Option Bytes) (argument_at index arguments) (None Bytes))
      (hordered : Equal Bool (leq_nat start end) True)
      (hbounded : Equal Bool (leq_nat end (bytes_nat_length argument)) True)
    : Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (None ArgLocation) =
  J
    (λlookup _.
      Equal
        (Option ArgLocation)
        (match lookup {
          None ↦ None ArgLocation;
          Some found ↦
            match leq_nat start end {
              False ↦ None ArgLocation;
              True ↦
                match leq_nat end (bytes_nat_length found) {
                  False ↦ None ArgLocation;
                  True ↦ Some ArgLocation (MkArgLocation index start end)
                }
            }
        })
        (None ArgLocation))
    Proved
    (J (λright _. Equal (Option Bytes) right (argument_at index arguments)) Refl hmissing)

pub proof refuses_unordered_endpoints for argument_slice_location
      (index : Nat)
      (start : Nat)
      (end : Nat)
      (arguments : List Bytes)
      (argument : Bytes)
      (hargument : Equal (Option Bytes) (argument_at index arguments) (Some Bytes argument))
      (hunordered : Equal Bool (leq_nat start end) False)
      (hbounded : Equal Bool (leq_nat end (bytes_nat_length argument)) True)
    : Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (None ArgLocation) =
  J
    (λlookup _.
      Equal
        (Option ArgLocation)
        (match lookup {
          None ↦ None ArgLocation;
          Some found ↦
            match leq_nat start end {
              False ↦ None ArgLocation;
              True ↦
                match leq_nat end (bytes_nat_length found) {
                  False ↦ None ArgLocation;
                  True ↦ Some ArgLocation (MkArgLocation index start end)
                }
            }
        })
        (None ArgLocation))
    (J
      (λordered _.
        Equal
          (Option ArgLocation)
          (match ordered {
            False ↦ None ArgLocation;
            True ↦
              match leq_nat end (bytes_nat_length argument) {
                False ↦ None ArgLocation;
                True ↦ Some ArgLocation (MkArgLocation index start end)
              }
          })
          (None ArgLocation))
      Proved
      (J (λright _. Equal Bool right (leq_nat start end)) Refl hunordered))
    (J (λright _. Equal (Option Bytes) right (argument_at index arguments)) Refl hargument)

pub proof refuses_out_of_bounds for argument_slice_location
      (index : Nat)
      (start : Nat)
      (end : Nat)
      (arguments : List Bytes)
      (argument : Bytes)
      (hargument : Equal (Option Bytes) (argument_at index arguments) (Some Bytes argument))
      (hordered : Equal Bool (leq_nat start end) True)
      (hout_of_bounds : Equal Bool (leq_nat end (bytes_nat_length argument)) False)
    : Equal
        (Option ArgLocation)
        (argument_slice_location index start end arguments)
        (None ArgLocation) =
  J
    (λlookup _.
      Equal
        (Option ArgLocation)
        (match lookup {
          None ↦ None ArgLocation;
          Some found ↦
            match leq_nat start end {
              False ↦ None ArgLocation;
              True ↦
                match leq_nat end (bytes_nat_length found) {
                  False ↦ None ArgLocation;
                  True ↦ Some ArgLocation (MkArgLocation index start end)
                }
            }
        })
        (None ArgLocation))
    (J
      (λordered _.
        Equal
          (Option ArgLocation)
          (match ordered {
            False ↦ None ArgLocation;
            True ↦
              match leq_nat end (bytes_nat_length argument) {
                False ↦ None ArgLocation;
                True ↦ Some ArgLocation (MkArgLocation index start end)
              }
          })
          (None ArgLocation))
      (J
        (λbounded _.
          Equal
            (Option ArgLocation)
            (match bounded {
              False ↦ None ArgLocation;
              True ↦ Some ArgLocation (MkArgLocation index start end)
            })
            (None ArgLocation))
        Proved
        (J
          (λright _. Equal Bool right (leq_nat end (bytes_nat_length argument)))
          Refl
          hout_of_bounds))
      (J (λright _. Equal Bool right (leq_nat start end)) Refl hordered))
    (J (λright _. Equal (Option Bytes) right (argument_at index arguments)) Refl hargument)
```

## 3. Design notes

The raw ABI projection and parsing view meet at `Bytes`, not `String`.
`ArgLocation` remains the sole argument byte-range carrier, while lengths are
computed from the total structural byte view.

The completeness law is a proposition-level existential eliminator. Its
caller-chosen proposition receives one argument together with that argument's
lookup fact, both range guards, and the exact `MkArgLocation` identity; it does
not expose a second data-returning API.

## 4. Trust & derivation

**Public API:** `process_arguments`, `replace_process_arguments`,
`process_arguments::round_trip`, `process_argument_at`, `argument_at`,
`argument_slice_location`, `argument_slice_location::soundness`,
`argument_slice_location::completeness`, and the three attached refusal proofs.
The identity-shaped `argument_bytes_at` helper and proof helpers stay private.

All declarations are transparent checked terms over landed `ProcessInput`,
`List`, `Bytes`, and `ArgLocation`. The package declares no primitive,
postulate, opaque constant, or `Axiom`; its `trusted_base()` delta is zero.
