# Lexer — streaming a DFA over cursor elements

A cursor's bounded element list provides an input word for a deterministic
automaton. This bridge runs the automaton directly on the cursor, without
materialising that list, and proves both paths return the same state and
acceptance result.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A DFA normally reads a `List el`, whereas a parser obtains symbols from a
cursor. The bounded scan uses the cursor's own remaining count for fuel: a
stuck or prematurely empty cursor still terminates, with no progress law
required for this bridge. Lexer imports Dfa and Cursor in one direction;
neither provider knows about lexing.

## 2. Definition

The public runner accepts a supplied starting state. Its private fuelled scan
reads one `cursor_peek` at a time, steps the DFA, advances the cursor, and
repeats only while fuel remains. The acceptance operation chooses the DFA's
start state and applies its final-state test to the result. The two proved
public bridges follow immediately after those operations in the checked
source; proof helpers are private.

```ken
import Algorithm.FormalLanguages.Dfa (Dfa, accepts, final, run, start, step)

import Capability.Parsing.Cursor
  (CursorOps, cursor_advance, cursor_elements, cursor_peek, cursor_remaining, cursor_take)

import Core.Logic.Transport (cong)

pub fn dfa_cursor_run
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (s : q)
      (cur : c)
    : q =
  dfa_cursor_run_fuel q c el loc d ops (cursor_remaining c el loc ops cur) s cur

pub fn dfa_cursor_accepts
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (cur : c)
    : Bool =
  final q el d (dfa_cursor_run q c el loc d ops (start q el d) cur)

pub theorem dfa_cursor_run_elements
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (s : q)
      (cur : c)
    : Equal q
        (dfa_cursor_run q c el loc d ops s cur)
        (run q el d s (cursor_elements c el loc ops cur)) =
  dfa_cursor_run_fuel_take q c el loc d ops (cursor_remaining c el loc ops cur) s cur

pub theorem dfa_cursor_accepts_elements
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (cur : c)
    : Equal Bool
        (dfa_cursor_accepts q c el loc d ops cur)
        (accepts q el d (cursor_elements c el loc ops cur)) =
  cong
    q
    Bool
    (dfa_cursor_run q c el loc d ops (start q el d) cur)
    (run q el d (start q el d) (cursor_elements c el loc ops cur))
    (final q el d)
    (dfa_cursor_run_elements q c el loc d ops (start q el d) cur)

fn dfa_cursor_run_fuel
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (fuel : Nat)
      (s : q)
      (cur : c)
    : q =
  match fuel {
    Zero ↦ s;
    Suc fuel2 ↦
      match cursor_peek c el loc ops cur {
        None ↦ s;
        Some x ↦
          dfa_cursor_run_fuel
            q
            c
            el
            loc
            d
            ops
            fuel2
            (step q el d s x)
            (cursor_advance c el loc ops cur)
      }
  }

fn dfa_cursor_run_step
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (fuel : Nat)
      (s : q)
      (cur : c)
      (selected : Option el)
    : q =
  match selected {
    None ↦ s;
    Some x ↦
      dfa_cursor_run_fuel
        q
        c
        el
        loc
        d
        ops
        fuel
        (step q el d s x)
        (cursor_advance c el loc ops cur)
  }

theorem dfa_cursor_run_step_take
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (fuel : Nat)
      (ih : (s : q)
        → (cur : c)
        → Equal
        q
        (dfa_cursor_run_fuel q c el loc d ops fuel s cur)
        (run q el d s (cursor_take c el loc ops fuel cur)))
      (s : q)
      (cur : c)
      (selected : Option el)
    : Equal q
        (dfa_cursor_run_step q c el loc d ops fuel s cur selected)
        (run q el d s (dfa_cursor_take_step c el loc ops fuel cur selected)) =
  match selected {
    None ↦ Refl;
    Some x ↦ ih (step q el d s x) (cursor_advance c el loc ops cur)
  }

theorem dfa_cursor_run_fuel_take
      (q : Type)
      (c : Type)
      (el : Type)
      (loc : Type)
      (d : Dfa q el)
      (ops : CursorOps c el loc)
      (fuel : Nat)
    : (s : q)
      → (cur : c)
      → Equal q
        (dfa_cursor_run_fuel q c el loc d ops fuel s cur)
        (run q el d s (cursor_take c el loc ops fuel cur)) =
  match fuel {
    Zero ↦ λs. λcur. Refl;
    Suc fuel2 ↦
      λs.
        λcur.
          dfa_cursor_run_step_take
            q
            c
            el
            loc
            d
            ops
            fuel2
            (dfa_cursor_run_fuel_take q c el loc d ops fuel2)
            s
            cur
            (cursor_peek c el loc ops cur)
  }

fn dfa_cursor_take_step
      (c : Type)
      (el : Type)
      (loc : Type)
      (ops : CursorOps c el loc)
      (fuel : Nat)
      (cur : c)
      (selected : Option el)
    : List el =
  match selected {
    None ↦ Nil el;
    Some x ↦ Cons el x (cursor_take c el loc ops fuel (cursor_advance c el loc ops cur))
  }
```

## 3. Using it

The existing argument-byte cursor is one instance. This example checks a DFA
that toggles acceptance once per byte; `arg_cursor_start` normalises empty
arguments before Lexer observes the first byte.

```ken example
import Capability.Parsing.Cursor (ArgCursor, ArgLocation, arg_cursor_ops, arg_cursor_start)

import Algorithm.FormalLanguages.Dfa (MkDfa)

import Core.Classes.LawfulClasses (bool_not)

const lexer_even_length : Dfa Bool UInt8 = MkDfa Bool UInt8 (λs. λx. bool_not s) True (λs. s)

const lexer_two_bytes : ArgCursor =
  arg_cursor_start (Cons Bytes (bytes_encode "aa") (Nil Bytes))

const lexer_accepts_two_bytes : Bool =
  dfa_cursor_accepts
    Bool
    ArgCursor
    UInt8
    ArgLocation
    lexer_even_length
    arg_cursor_ops
    lexer_two_bytes
```

## 4. Laws & proofs

`dfa_cursor_run_elements` inducts on exactly the fuel used by `cursor_take`.
When a peek returns `None`, both computations stop; on `Some`, their tails
use the same advanced cursor and stepped state. No `CursorLaws` argument is
needed. `dfa_cursor_accepts_elements` transports the state equality through
`final` after choosing the automaton's `start` state. Both checked theorem
bodies are in the Definition fence above, adjacent to their operations.

## 5. Design notes

The runner streams without building a list, but its bridge equates it with
the bounded list that `cursor_elements` constructs. It does not return the
end cursor or the longest accepted prefix; maximal munch needs a separate
maximality law. It makes no location claim, no cross-instance cursor
comparison, and no claim that an unlawful cursor traverses an unbounded
stream. The cursor's own `cursor_elements_peek_some` states the stronger
unfold equation only when advance progress is proved.

## 6. References

- [Lexical analysis (Wikipedia)](https://en.wikipedia.org/wiki/Lexical_analysis)
  introduces the relationship between characters, scanners, and tokens.
- Aho, Lam, Sethi, and Ullman, *Compilers: Principles, Techniques, and Tools*,
  2nd edition, describes DFA-based scanners and their use in compilers.

## 7. Trust & derivation

The contract is
[formal languages §6](../../../../spec/50-stdlib/61-formal-languages.md).
The seven runtime examples are in the lexer bridge
[seed](../../../../conformance/stdlib/formal-languages/seed-lexer-bridge.md).

| Task | Location |
|---|---|
| Stream from supplied state | Definition: `dfa_cursor_run` |
| Decide from the start state | Definition: `dfa_cursor_accepts` |
| Verify state/list agreement | Definition: `dfa_cursor_run_elements` |
| Verify acceptance/list agreement | Definition: `dfa_cursor_accepts_elements` |

The derivation uses Dfa's checked `run`/`step`/`final`, Cursor's bounded
`cursor_take`/`cursor_elements` and `peek`/`advance`, plus ordinary equality
transport. No new trusted rule, Axiom, primitive or postulate is introduced:
`trusted_base()` after the package equals its imported-provider baseline.
The two generic bridge proofs cover all `CursorOps`; the runtime byte seed
exercises the separate argument-cursor instance without asserting a kernel
proof about conversion-opaque byte values.
