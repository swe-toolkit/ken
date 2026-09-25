# `Tooling.Testing.Property` — deterministic finite property checks

`Tooling.Testing.Property` exercises decidable predicates over explicit finite
samples and reports the first counterexample.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

A small property runner is useful even without randomness. An explicit finite
sample list makes every run deterministic, bounded, and reproducible. The
runner evaluates a `Bool` predicate and returns either success or the first
sample that falsified it.

The concrete witnesses here are computations. Private checked laws additionally
state what successful and failing runs imply for every finite sample list.

## Definition

`Gen` exposes only construction from a sample list and mapping. Runner outcomes
use the ordinary error-biased `Result`: an error is the first counterexample,
while success carries `Unit`.

```ken
import Data.Collections.Derived (length, map, nth)

data Gen a = MkGen (List a)

fn gen_from_list (a : Type) (samples : List a) : Gen a = MkGen a samples

fn gen_samples (a : Type) (generator : Gen a) : List a =
  match generator {
    MkGen samples ↦ samples
  }

fn gen_map (a : Type) (b : Type) (f : a → b) (generator : Gen a) : Gen b =
  gen_from_list b (map a b f (gen_samples a generator))

fn check_samples (a : Type) (samples : List a) (predicate : a → Bool) : Result a Unit =
  match samples {
    Nil ↦ Ok a Unit MkUnit;
    Cons sample rest ↦
      match predicate sample {
        True ↦ check_samples a rest predicate;
        False ↦ Err a Unit sample
      }
  }

fn check (a : Type) (generator : Gen a) (predicate : a → Bool) : Result a Unit =
  check_samples a (gen_samples a generator) predicate
```

The byte generator starts from the structural `List UInt8` view and maps the
landed total conversion into `Bytes`. Its five samples include the empty input,
both boundary singletons, and two multi-byte inputs.

```ken
const byte_sample_lists : List (List UInt8) =
  Cons
    (List UInt8)
    (Nil UInt8)
    (Cons
      (List UInt8)
      (Cons UInt8 0 (Nil UInt8))
      (Cons
        (List UInt8)
        (Cons UInt8 255 (Nil UInt8))
        (Cons
          (List UInt8)
          (Cons UInt8 0 (Cons UInt8 255 (Nil UInt8)))
          (Cons
            (List UInt8)
            (Cons UInt8 1 (Cons UInt8 127 (Cons UInt8 255 (Nil UInt8))))
            (Nil (List UInt8))))))

const gen_byte_lists : Gen (List UInt8) = gen_from_list (List UInt8) byte_sample_lists

const gen_bytes : Gen Bytes = gen_map (List UInt8) Bytes list_to_bytes gen_byte_lists
```

The cursor slice contains only the operations used by the progress check. It
views a `Bytes` value structurally, peeks at the head, advances by dropping one
element, and counts the remaining elements.

```ken
data ByteCursor = MkByteCursor (List UInt8)

fn byte_cursor_start (input : Bytes) : ByteCursor = MkByteCursor (bytes_to_list input)

fn byte_cursor_remaining (cursor : ByteCursor) : Nat =
  match cursor {
    MkByteCursor bytes ↦ length UInt8 bytes
  }

fn byte_cursor_peek (cursor : ByteCursor) : Option UInt8 =
  match cursor {
    MkByteCursor bytes ↦
      match bytes {
        Nil ↦ None UInt8;
        Cons byte rest ↦ Some UInt8 byte
      }
  }

fn byte_cursor_advance (cursor : ByteCursor) : ByteCursor =
  match cursor {
    MkByteCursor bytes ↦
      match bytes {
        Nil ↦ MkByteCursor (Nil UInt8);
        Cons byte rest ↦ MkByteCursor rest
      }
  }

fn byte_cursor_stuck_advance (cursor : ByteCursor) : ByteCursor = cursor

fn property_nat_lt (left : Nat) (right : Nat) : Bool =
  match right {
    Zero ↦ False;
    Suc right2 ↦
      match left {
        Zero ↦ True;
        Suc left2 ↦ property_nat_lt left2 right2
      }
  }

fn cursor_progress_with (advance : ByteCursor → ByteCursor) (input : Bytes) : Bool =
  let cursor : ByteCursor =
    byte_cursor_start input
  in
    match byte_cursor_peek cursor {
      None ↦ True;
      Some byte ↦
        property_nat_lt (byte_cursor_remaining (advance cursor)) (byte_cursor_remaining cursor)
    }

fn cursor_progress (input : Bytes) : Bool = cursor_progress_with byte_cursor_advance input

fn cursor_stuck_progress (input : Bytes) : Bool =
  cursor_progress_with byte_cursor_stuck_advance input
```

## Using it

`check` stops at the first false predicate. These helpers inspect its ordinary
`Result` value without turning a test outcome into a proof.

```ken
fn property_result_is_held (a : Type) (outcome : Result a Unit) : Bool =
  match outcome {
    Err counterexample ↦ False;
    Ok unit ↦ True
  }

fn property_uint8_eq (left : UInt8) (right : UInt8) : Bool =
  eq_int (uint8_to_int left) (uint8_to_int right)

fn property_list_uint8_eq (left : List UInt8) (right : List UInt8) : Bool =
  match left {
    Nil ↦
      match right {
        Nil ↦ True;
        Cons head tail ↦ False
      };
    Cons left_head left_tail ↦
      match right {
        Nil ↦ False;
        Cons right_head right_tail ↦
          match property_uint8_eq left_head right_head {
            True ↦ property_list_uint8_eq left_tail right_tail;
            False ↦ False
          }
      }
  }

fn property_bytes_eq (left : Bytes) (right : Bytes) : Bool =
  property_list_uint8_eq (bytes_to_list left) (bytes_to_list right)

fn property_result_failed_with
      (a : Type) (eq : a → a → Bool) (expected : a) (outcome : Result a Unit)
    : Bool =
  match outcome {
    Err counterexample ↦ eq counterexample expected;
    Ok unit ↦ False
  }

fn reject_every_byte_sample (input : Bytes) : Bool = False

const empty_byte_sample : Bytes = list_to_bytes (Nil UInt8)

const zero_byte_sample : Bytes = list_to_bytes (Cons UInt8 0 (Nil UInt8))

const first_counterexample_witness : Bool =
  property_result_failed_with
    Bytes
    property_bytes_eq
    empty_byte_sample
    (check Bytes gen_bytes reject_every_byte_sample)
```

## Laws & proofs

The runner laws are private checked terms. Soundness and completeness relate a
successful run to every successful `nth` lookup. The counterexample law recovers
an exact lookup and false predicate result, together with the fact that every
strictly earlier lookup passed. The two coherence laws reduce `check` and
`gen_samples` through `gen_from_list`.

The real cursor check succeeds across all five generated byte strings. The
stuck-advance mutant passes the empty input and then reaches the non-empty arm,
so the runner reports the zero singleton as its first counterexample. The three
concrete witnesses remain executable `Bool` computations rather than proof
terms.

```ken
theorem property_some_injective
      (a : Type) (left : a) (right : a) (same : Equal (Option a) (Some a left) (Some a right))
    : Equal a left right =
  same

theorem property_nth_zero_cons
      (a : Type) (sample : a) (rest : List a) (v : a)
    : Equal (Option a) (nth a Zero (Cons a sample rest)) (Some a v) → Equal a sample v =
  λhat. property_some_injective a sample v hat

theorem property_nth_zero_cons_self
      (a : Type) (sample : a) (rest : List a)
    : Equal (Option a) (nth a Zero (Cons a sample rest)) (Some a sample) =
  Refl

theorem property_nth_suc_cons
      (a : Type) (sample : a) (rest : List a) (i : Nat) (v : a)
    : Equal (Option a) (nth a (Suc i) (Cons a sample rest)) (Some a v)
      → Equal (Option a) (nth a i rest) (Some a v) =
  λhat. hat

theorem property_nth_suc_cons_lift
      (a : Type)
      (sample : a)
      (rest : List a)
      (i : Nat)
      (v : a)
      (hat : Equal (Option a) (nth a i rest) (Some a v))
    : Equal (Option a) (nth a (Suc i) (Cons a sample rest)) (Some a v) =
  hat

theorem property_err_injective
      (a : Type)
      (left : a)
      (right : a)
      (same : Equal (Result a Unit) (Err a Unit left) (Err a Unit right))
    : Equal a left right =
  same

theorem property_false_counterexample_same
      (a : Type)
      (sample : a)
      (rest : List a)
      (predicate : a → Bool)
      (x : a)
      (hfailure : Equal
        (Result a Unit)
        (check_samples a (Cons a sample rest) predicate)
        (Err a Unit x))
      (hsample : Equal Bool (predicate sample) False)
    : Equal a sample x =
  property_err_injective
    a
    sample
    x
    (J
      (λchoice _.
        Equal
          (Result a Unit)
          (match choice {
            True ↦ check_samples a rest predicate;
            False ↦ Err a Unit sample
          })
          (Err a Unit x))
      hfailure
      hsample)

theorem property_true_counterexample_tail
      (a : Type)
      (sample : a)
      (rest : List a)
      (predicate : a → Bool)
      (x : a)
      (hfailure : Equal
        (Result a Unit)
        (check_samples a (Cons a sample rest) predicate)
        (Err a Unit x))
      (hsample : Equal Bool (predicate sample) True)
    : Equal (Result a Unit) (check_samples a rest predicate) (Err a Unit x) =
  J
    (λchoice _.
      Equal
        (Result a Unit)
        (match choice {
          True ↦ check_samples a rest predicate;
          False ↦ Err a Unit sample
        })
        (Err a Unit x))
    hfailure
    hsample

theorem check_samples_soundness
      (a : Type) (samples : List a) (predicate : a → Bool)
    : (i : Nat)
      → (v : a)
      → Equal (Result a Unit) (check_samples a samples predicate) (Ok a Unit MkUnit)
      → Equal (Option a) (nth a i samples) (Some a v)
      → Equal Bool (predicate v) True =
  match samples {
    Nil ↦ λi. λv. λhsuccess. λhat. absurd hat;
    Cons sample rest ↦
      λi.
        match i {
          Zero ↦
            λv.
              match predicate sample eqn : hsample {
                False ↦
                  λhsuccess.
                    λhat.
                      absurd
                        (J
                          (λchoice _.
                            Equal
                              (Result a Unit)
                              (match choice {
                                True ↦ check_samples a rest predicate;
                                False ↦ Err a Unit sample
                              })
                              (Ok a Unit MkUnit))
                          hsuccess
                          hsample);
                True ↦
                  λhsuccess.
                    λhat.
                      J
                        (λvalue _. Equal Bool (predicate value) True)
                        hsample
                        (property_nth_zero_cons a sample rest v hat)
              };
          Suc i2 ↦
            λv.
              match predicate sample eqn : hsample {
                False ↦
                  λhsuccess.
                    λhat.
                      absurd
                        (J
                          (λchoice _.
                            Equal
                              (Result a Unit)
                              (match choice {
                                True ↦ check_samples a rest predicate;
                                False ↦ Err a Unit sample
                              })
                              (Ok a Unit MkUnit))
                          hsuccess
                          hsample);
                True ↦
                  λhsuccess.
                    λhat.
                      check_samples_soundness
                        a
                        rest
                        predicate
                        i2
                        v
                        (J
                          (λchoice _.
                            Equal
                              (Result a Unit)
                              (match choice {
                                True ↦ check_samples a rest predicate;
                                False ↦ Err a Unit sample
                              })
                              (Ok a Unit MkUnit))
                          hsuccess
                          hsample)
                        (property_nth_suc_cons a sample rest i2 v hat)
              }
        }
  }

proof soundness for check_samples
      (a : Type) (samples : List a) (predicate : a → Bool)
    : (i : Nat)
      → (v : a)
      → Equal (Result a Unit) (check_samples a samples predicate) (Ok a Unit MkUnit)
      → Equal (Option a) (nth a i samples) (Some a v)
      → Equal Bool (predicate v) True =
  check_samples_soundness a samples predicate

proof success_complete for check_samples
      (a : Type) (samples : List a) (predicate : a → Bool)
    : ((i : Nat)
        → (v : a)
        → Equal
        (Option a)
        (nth a i samples)
        (Some a v)
        → Equal
        Bool
        (predicate v)
        True)
      → Equal (Result a Unit) (check_samples a samples predicate) (Ok a Unit MkUnit) =
  match samples {
    Nil ↦ λall_pass. Proved;
    Cons sample rest ↦
      λall_pass.
        match predicate sample eqn : hsample {
          False ↦
            absurd
              (J
                (λright _. Equal Bool right True)
                (all_pass Zero sample (property_nth_zero_cons_self a sample rest))
                hsample);
          True ↦
            J
              (λchoice _.
                Equal
                  (Result a Unit)
                  (match choice {
                    True ↦ check_samples a rest predicate;
                    False ↦ Err a Unit sample
                  })
                  (Ok a Unit MkUnit))
              ((proof success_complete for check_samples)
                a
                rest
                predicate
                (λi.
                  λv.
                    λhat.
                      all_pass (Suc i) v (property_nth_suc_cons_lift a sample rest i v hat)))
              (J (λright _. Equal Bool right (predicate sample)) Refl hsample)
        }
  }

proof first_counterexample for check_samples
      (a : Type) (samples : List a) (predicate : a → Bool) (x : a)
    : Equal (Result a Unit) (check_samples a samples predicate) (Err a Unit x)
      → (goal : Prop)
      → ((i : Nat)
          → Equal
          (Option a)
          (nth a i samples)
          (Some a x)
          → Equal
          Bool
          (predicate x)
          False
          → ((j : Nat)
            → (v : a)
            → Equal
            Bool
            (property_nat_lt j i)
            True
            → Equal
            (Option a)
            (nth a j samples)
            (Some a v)
            → Equal
            Bool
            (predicate v)
            True)
          → goal)
      → goal =
  match samples {
    Nil ↦ λhfailure. λgoal. λrecover. absurd hfailure;
    Cons sample rest ↦
      λhfailure.
        match predicate sample eqn : hsample {
          False ↦
            λgoal.
              λrecover.
                recover
                  Zero
                  (J
                    (λvalue _.
                      Equal (Option a) (nth a Zero (Cons a sample rest)) (Some a value))
                    (property_nth_zero_cons_self a sample rest)
                    (property_false_counterexample_same
                      a
                      sample
                      rest
                      predicate
                      x
                      hfailure
                      hsample))
                  (J
                    (λvalue _. Equal Bool (predicate value) False)
                    hsample
                    (property_false_counterexample_same
                      a
                      sample
                      rest
                      predicate
                      x
                      hfailure
                      hsample))
                  (λj. λv. λhearlier. λhat. absurd hearlier);
          True ↦
            λgoal.
              λrecover.
                (proof first_counterexample for check_samples)
                  a
                  rest
                  predicate
                  x
                  (property_true_counterexample_tail a sample rest predicate x hfailure hsample)
                  goal
                  (λi.
                    λhat.
                      λhfalse.
                        λearlier.
                          recover
                            (Suc i)
                            (property_nth_suc_cons_lift a sample rest i x hat)
                            hfalse
                            (λj.
                              match j {
                                Zero ↦
                                  λv.
                                    λhearlier.
                                      λhat2.
                                        J
                                          (λvalue _. Equal Bool (predicate value) True)
                                          hsample
                                          (property_nth_zero_cons a sample rest v hat2);
                                Suc j2 ↦
                                  λv.
                                    λhearlier.
                                      λhat2.
                                        earlier
                                          j2
                                          v
                                          hearlier
                                          (property_nth_suc_cons a sample rest j2 v hat2)
                              }))
        }
  }

proof from_list for check
      (a : Type) (samples : List a) (predicate : a → Bool)
    : Equal
        (Result a Unit)
        (check a (gen_from_list a samples) predicate)
        (check_samples a samples predicate) =
  Refl

proof from_list for gen_samples
      (a : Type) (samples : List a)
    : Equal (List a) (gen_samples a (gen_from_list a samples)) samples =
  Refl

const cursor_progress_witness : Bool =
  property_result_is_held Bytes (check Bytes gen_bytes cursor_progress)

const cursor_stuck_counterexample_witness : Bool =
  property_result_failed_with
    Bytes
    property_bytes_eq
    zero_byte_sample
    (check Bytes gen_bytes cursor_stuck_progress)
```

## Design notes

The deterministic representation keeps failure order stable and makes the
first counterexample reproducible. `gen_map` is the only composition operation:
binding, shrinking, random seeds, effects, and size parameters are deliberately
absent.

The cursor predicate has a live `Some` branch. Replacing its advance operation
with the identity makes that branch compute `False`, which the mutant witness
observes on the first non-empty generated value.

## References

- Claessen and Hughes, *QuickCheck: A Lightweight Tool for Random Testing of
  Haskell Programs*, ICFP 2000 — <https://doi.org/10.1145/351240.351266> —
  orientation on generator-driven property testing; this package deliberately
  uses finite deterministic samples instead of randomness.
- Wikipedia, *Property testing* — <https://en.wikipedia.org/wiki/Property_testing>
  — overview of checking general behavior over generated inputs.

## Trust & derivation

The package declares no public surface. Its implementation includes `Gen`,
`gen_from_list`, `gen_samples`, `gen_map`, `check`, `gen_bytes`, and the three
executable witnesses. Runner outcomes reuse the prelude's `Result a Unit`
rather than introducing another result carrier. The implementation derives
from the prelude's `List`, `Result`, `Unit`, `Nat`, `Bytes`, and `UInt8` values
together with the total `Bytes`/`List UInt8` view.

The declared `trusted_base()` delta is **zero**. This package introduces no
primitive, postulate, axiom, proof hole, effect, or assumed proposition. The
cursor check consumes no proof-level cursor law; it evaluates only concrete
values.
