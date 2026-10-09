# `Regex` — checked regular languages and derivatives

Regular expressions describe finite-word languages independently of the
Boolean matcher that decides them. Every matcher law is checked over the
exported denotation and derivative, without an alphabet-finiteness assumption.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

`Regex a` denotes a proposition over each finite `List a` word. Its six
constructors cover failure, the empty word, one symbol, choice, ordered
concatenation, and finite repetition. Neither forming a regex nor stating its
language needs equality on `a`. When a caller supplies a `DecEq a` dictionary,
`regex_matches` decides one word's membership by deriving the expression
for each symbol and checking nullability at the end.

## 2. Definition

`Split` retains a concatenation's two words and their language proofs;
`Pieces` retains a finite sequence of words and a proof for every piece.
These Type-sorted witnesses are truncated when a `regex_lang` predicate
enters Ω. The shared `list_concat` and `list_all` operations are imported
from `Data.Collections.Derived`; the Boolean truth lemmas are imported from
`Core.Classes.LawfulClasses`, not redefined for this matcher.

`regex_matches` decides a finite word by deriving for each symbol, then
checking nullability. `deriv` returns an expression for a left quotient;
`nullable` checks the empty word structurally. The matcher and derivative
require an explicit decidable equality dictionary; nullability does not.

```ken
import Data.Collections.Derived (list_append, list_concat, list_all)

import Core.Logic.And (And, Both)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (cong, trans)

import Core.Classes.LawfulClasses
  (DecEq, bool_or, bool_and, or_left, or_right, or_cases, and_true, and_cases)

pub data Regex a =
  Fail
  | Eps
  | Sym a
  | Alt (Regex a) (Regex a)
  | Cat (Regex a) (Regex a)
  | Star (Regex a)

export Fail, Eps, Sym, Alt, Cat, Star

pub data Split (a : Type) (p : List a → Omega) (r : List a → Omega) (w : List a) : Type where {
  MkSplit :
    (u : List a)
    → (v : List a)
    → Equal (List a) (list_append a u v) w
    → p u
    → r v
    → Split a p r w
}

export MkSplit

pub data Pieces (a : Type) (p : List a → Omega) (w : List a) : Type where {
  MkPieces :
    (ws : List (List a))
    → Equal (List a) (list_concat a ws) w
    → list_all (List a) p ws
    → Pieces a p w
}

export MkPieces

pub fn regex_lang (a : Type) (r : Regex a) : List a → Omega =
  match r {
    Fail ↦ λw. Bottom;
    Eps ↦ λw. Equal (List a) w (Nil a);
    Sym c ↦
      λw.
        match w {
          Nil ↦ Bottom;
          Cons x rest ↦ (‖ And (Equal a x c) (Equal (List a) rest (Nil a)) ‖)
        };
    Alt r1 r2 ↦ λw. (‖ Or (regex_lang a r1 w) (regex_lang a r2 w) ‖);
    Cat r1 r2 ↦ λw. (‖ Split a (regex_lang a r1) (regex_lang a r2) w ‖);
    Star r1 ↦ λw. (‖ Pieces a (regex_lang a r1) w ‖)
  }

pub fn regex_matches (a : Type) (d : DecEq a) (r : Regex a) (w : List a) : Bool =
  match w {
    Nil ↦ nullable a r;
    Cons x rest ↦ regex_matches a d (deriv a d x r) rest
  }

pub fn deriv (a : Type) (d : DecEq a) (x : a) (r : Regex a) : Regex a =
  match r {
    Fail ↦ Fail a;
    Eps ↦ Fail a;
    Sym c ↦ guard a (d.eq x c) (Eps a);
    Alt r1 r2 ↦ Alt a (deriv a d x r1) (deriv a d x r2);
    Cat r1 r2 ↦ Alt a (Cat a (deriv a d x r1) r2) (guard a (nullable a r1) (deriv a d x r2));
    Star r1 ↦ Cat a (deriv a d x r1) (Star a r1)
  }

pub fn nullable (a : Type) (r : Regex a) : Bool =
  match r {
    Fail ↦ False;
    Eps ↦ True;
    Sym c ↦ False;
    Alt r1 r2 ↦ bool_or (nullable a r1) (nullable a r2);
    Cat r1 r2 ↦ bool_and (nullable a r1) (nullable a r2);
    Star r1 ↦ True
  }

fn guard (a : Type) (b : Bool) (t : Regex a) : Regex a =
  match b {
    True ↦ t;
    False ↦ Fail a
  }
```

## 3. Using it

With Boolean symbols, the canonical decidable-equality dictionary distinguishes
`True` from `False`. `Sym True` therefore accepts exactly the one-symbol word
containing `True`.

```ken example
const one_true : Regex Bool = Sym Bool True

const one_true_matches : Bool =
  regex_matches Bool DecEq_instance_Bool one_true (Cons Bool True (Nil Bool))

theorem one_true_matches_proof : Equal Bool one_true_matches True = Proved
```

## 4. Laws & proofs

The two matcher laws are the external guarantee: a `True` result produces a
language proof, and any language proof forces a `True` result. Their checked
bodies delegate to induction on the input word. Empty words use the nullable
laws; nonempty words use the derivative laws on the same exported operations.

```ken
pub theorem regex_matches_sound
      (a : Type) (d : DecEq a) (r : Regex a) (w : List a)
    : Equal Bool (regex_matches a d r w) True → regex_lang a r w =
  matches_sound_from a d w r

pub theorem regex_matches_complete
      (a : Type) (d : DecEq a) (r : Regex a) (w : List a)
    : regex_lang a r w → Equal Bool (regex_matches a d r w) True =
  matches_complete_from a d w r
```

The private proof sequence first transports list and Boolean evidence, then
proves nullability, both directions of the derivative, and finally the
word-induction helpers. In concatenation completeness, the `Cat` split handles
both an empty left word and a nonempty left word. In repetition completeness,
`star_split` skips any leading empty pieces before consuming the first symbol;
the trailing `Star` retains the rest of the finite decomposition. No private
lemma substitutes a second matcher for the one exported above.

```ken
fn list_head_or (a : Type) (dflt : a) (l : List a) : a =
  match l {
    Nil ↦ dflt;
    Cons y ys ↦ y
  }

fn list_tail (a : Type) (l : List a) : List a =
  match l {
    Nil ↦ Nil a;
    Cons y ys ↦ ys
  }

theorem guard_intro
      (a : Type) (b : Bool) (t : Regex a) (w : List a)
    : Equal Bool b True → regex_lang a t w → regex_lang a (guard a b t) w =
  match b {
    True ↦ λe. λh. h;
    False ↦ λe. λh. absurd e
  }

theorem guard_elim
      (a : Type) (b : Bool) (t : Regex a) (w : List a) (g : Omega)
    : regex_lang a (guard a b t) w → (Equal Bool b True → regex_lang a t w → g) → g =
  match b {
    True ↦ λh. λk. k Proved h;
    False ↦ λh. λk. absurd h
  }

pub theorem nullable_sound
      (a : Type) (r : Regex a)
    : Equal Bool (nullable a r) True → regex_lang a r (Nil a) =
  match r {
    Fail ↦ λh. absurd h;
    Eps ↦ λh. Proved;
    Sym c ↦ λh. absurd h;
    Alt r1 r2 ↦
      λh.
        or_cases
          (nullable a r1)
          (nullable a r2)
          (regex_lang a (Alt a r1 r2) (Nil a))
          h
          (λh1.
            trunc_intro
              (Inl
                (regex_lang a r1 (Nil a))
                (regex_lang a r2 (Nil a))
                (nullable_sound a r1 h1)))
          (λh2.
            trunc_intro
              (Inr
                (regex_lang a r1 (Nil a))
                (regex_lang a r2 (Nil a))
                (nullable_sound a r2 h2)));
    Cat r1 r2 ↦
      λh.
        and_cases
          (nullable a r1)
          (nullable a r2)
          (regex_lang a (Cat a r1 r2) (Nil a))
          h
          (λh1.
            λh2.
              trunc_intro
                (MkSplit
                  a
                  (regex_lang a r1)
                  (regex_lang a r2)
                  (Nil a)
                  (Nil a)
                  (Nil a)
                  Proved
                  (nullable_sound a r1 h1)
                  (nullable_sound a r2 h2)));
    Star r1 ↦
      λh. trunc_intro (MkPieces a (regex_lang a r1) (Nil a) (Nil (List a)) Proved Proved)
  }

theorem append_nil_left
      (a : Type) (u : List a) (v : List a)
    : Equal (List a) (list_append a u v) (Nil a) → Equal (List a) u (Nil a) =
  match u {
    Nil ↦ λe. Proved;
    Cons y ys ↦ λe. absurd e
  }

theorem append_nil_right
      (a : Type) (u : List a) (v : List a)
    : Equal (List a) (list_append a u v) (Nil a) → Equal (List a) v (Nil a) =
  match u {
    Nil ↦ λe. e;
    Cons y ys ↦ λe. absurd e
  }

pub theorem nullable_complete
      (a : Type) (r : Regex a)
    : regex_lang a r (Nil a) → Equal Bool (nullable a r) True =
  match r {
    Fail ↦ λh. absurd h;
    Eps ↦ λh. Proved;
    Sym c ↦ λh. absurd h;
    Alt r1 r2 ↦
      λh.
        elim_trunc
          (Equal Bool (nullable a (Alt a r1 r2)) True)
          (λo.
            match o {
              Inl h1 ↦ or_left (nullable a r1) (nullable a r2) (nullable_complete a r1 h1);
              Inr h2 ↦ or_right (nullable a r1) (nullable a r2) (nullable_complete a r2 h2)
            })
          h;
    Cat r1 r2 ↦
      λh.
        elim_trunc
          (Equal Bool (nullable a (Cat a r1 r2)) True)
          (λs.
            match s {
              MkSplit u v e pu pv ↦
                and_true
                  (nullable a r1)
                  (nullable a r2)
                  (nullable_complete
                    a
                    r1
                    (transport_any
                      (List a)
                      (regex_lang a r1)
                      u
                      (Nil a)
                      (append_nil_left a u v e)
                      pu))
                  (nullable_complete
                    a
                    r2
                    (transport_any
                      (List a)
                      (regex_lang a r2)
                      v
                      (Nil a)
                      (append_nil_right a u v e)
                      pv))
            })
          h;
    Star r1 ↦ λh. Proved
  }

theorem transport_any
      (t : Type) (p : t → Omega) (s : t) (u : t) (e : Equal t s u) (ps : p s)
    : p u =
  J (λu2 _. p u2) ps e

pub theorem deriv_sound
      (a : Type) (d : DecEq a) (x : a) (r : Regex a)
    : (w : List a) → regex_lang a (deriv a d x r) w → regex_lang a r (Cons a x w) =
  match r {
    Fail ↦ λw. λh. absurd h;
    Eps ↦ λw. λh. absurd h;
    Sym c ↦
      λw.
        λh.
          guard_elim
            a
            (d.eq x c)
            (Eps a)
            w
            (regex_lang a (Sym a c) (Cons a x w))
            h
            (λt.
              λnil.
                trunc_intro
                  (Both (Equal a x c) (Equal (List a) w (Nil a)) (d.sound x c t) nil));
    Alt r1 r2 ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (Alt a r1 r2) (Cons a x w))
            (λo.
              match o {
                Inl h1 ↦
                  trunc_intro
                    (Inl
                      (regex_lang a r1 (Cons a x w))
                      (regex_lang a r2 (Cons a x w))
                      (deriv_sound a d x r1 w h1));
                Inr h2 ↦
                  trunc_intro
                    (Inr
                      (regex_lang a r1 (Cons a x w))
                      (regex_lang a r2 (Cons a x w))
                      (deriv_sound a d x r2 w h2))
              })
            h;
    Cat r1 r2 ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (Cat a r1 r2) (Cons a x w))
            (λo.
              match o {
                Inl hs ↦
                  elim_trunc
                    (regex_lang a (Cat a r1 r2) (Cons a x w))
                    (λs.
                      match s {
                        MkSplit u v e p q ↦
                          trunc_intro
                            (MkSplit
                              a
                              (regex_lang a r1)
                              (regex_lang a r2)
                              (Cons a x w)
                              (Cons a x u)
                              v
                              (cong (List a) (List a) (list_append a u v) w (λl. Cons a x l) e)
                              (deriv_sound a d x r1 u p)
                              q)
                      })
                    hs;
                Inr hg ↦
                  guard_elim
                    a
                    (nullable a r1)
                    (deriv a d x r2)
                    w
                    (regex_lang a (Cat a r1 r2) (Cons a x w))
                    hg
                    (λn.
                      λq.
                        trunc_intro
                          (MkSplit
                            a
                            (regex_lang a r1)
                            (regex_lang a r2)
                            (Cons a x w)
                            (Nil a)
                            (Cons a x w)
                            Refl
                            (nullable_sound a r1 n)
                            (deriv_sound a d x r2 w q)))
              })
            h;
    Star r1 ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (Star a r1) (Cons a x w))
            (λs.
              match s {
                MkSplit u v e p q ↦
                  elim_trunc
                    (regex_lang a (Star a r1) (Cons a x w))
                    (λps.
                      match ps {
                        MkPieces ws ec allw ↦
                          trunc_intro
                            (MkPieces
                              a
                              (regex_lang a r1)
                              (Cons a x w)
                              (Cons (List a) (Cons a x u) ws)
                              (cong
                                (List a)
                                (List a)
                                (list_append a u (list_concat a ws))
                                w
                                (λl. Cons a x l)
                                (trans
                                  (List a)
                                  (list_append a u (list_concat a ws))
                                  (list_append a u v)
                                  w
                                  (cong
                                    (List a)
                                    (List a)
                                    (list_concat a ws)
                                    v
                                    (λl. list_append a u l)
                                    ec)
                                  e))
                              (trunc_intro
                                (Both
                                  (regex_lang a r1 (Cons a x u))
                                  (list_all (List a) (regex_lang a r1) ws)
                                  (deriv_sound a d x r1 u p)
                                  allw)))
                      })
                    q
              })
            h
  }

theorem cat_split
      (a : Type)
      (d : DecEq a)
      (x : a)
      (r1 : Regex a)
      (r2 : Regex a)
      (ih1 : (u : List a) → regex_lang a r1 (Cons a x u) → regex_lang a (deriv a d x r1) u)
      (ih2 : (w : List a) → regex_lang a r2 (Cons a x w) → regex_lang a (deriv a d x r2) w)
      (w : List a)
      (u : List a)
    : (v : List a)
      → Equal (List a) (list_append a u v) (Cons a x w)
      → regex_lang a r1 u
      → regex_lang a r2 v
      → regex_lang a (deriv a d x (Cat a r1 r2)) w =
  match u {
    Nil ↦
      λv.
        λe.
          λp.
            λq.
              trunc_intro
                (Inr
                  (regex_lang a (Cat a (deriv a d x r1) r2) w)
                  (regex_lang a (guard a (nullable a r1) (deriv a d x r2)) w)
                  (guard_intro
                    a
                    (nullable a r1)
                    (deriv a d x r2)
                    w
                    (nullable_complete a r1 p)
                    (ih2 w (transport_any (List a) (regex_lang a r2) v (Cons a x w) e q))));
    Cons y u2 ↦
      λv.
        λe.
          λp.
            λq.
              trunc_intro
                (Inl
                  (regex_lang a (Cat a (deriv a d x r1) r2) w)
                  (regex_lang a (guard a (nullable a r1) (deriv a d x r2)) w)
                  (trunc_intro
                    (MkSplit
                      a
                      (regex_lang a (deriv a d x r1))
                      (regex_lang a r2)
                      w
                      u2
                      v
                      (cong
                        (List a)
                        (List a)
                        (Cons a y (list_append a u2 v))
                        (Cons a x w)
                        (list_tail a)
                        e)
                      (ih1
                        u2
                        (transport_any
                          a
                          (λz. regex_lang a r1 (Cons a z u2))
                          y
                          x
                          (cong
                            (List a)
                            a
                            (Cons a y (list_append a u2 v))
                            (Cons a x w)
                            (list_head_or a y)
                            e)
                          p))
                      q)))
  }

theorem star_split
      (a : Type)
      (d : DecEq a)
      (x : a)
      (r1 : Regex a)
      (ih1 : (u : List a) → regex_lang a r1 (Cons a x u) → regex_lang a (deriv a d x r1) u)
      (ws : List (List a))
    : (w : List a)
      → Equal (List a) (list_concat a ws) (Cons a x w)
      → list_all (List a) (regex_lang a r1) ws
      → regex_lang a (Cat a (deriv a d x r1) (Star a r1)) w =
  match ws {
    Nil ↦ λw. λe. λallw. absurd e;
    Cons u rest ↦
      match u {
        Nil ↦
          λw.
            λe.
              λallw.
                elim_trunc
                  (regex_lang a (Cat a (deriv a d x r1) (Star a r1)) w)
                  (λb.
                    match b {
                      Both p0 allrest ↦ star_split a d x r1 ih1 rest w e allrest
                    })
                  allw;
        Cons y u2 ↦
          λw.
            λe.
              λallw.
                elim_trunc
                  (regex_lang a (Cat a (deriv a d x r1) (Star a r1)) w)
                  (λb.
                    match b {
                      Both p allrest ↦
                        trunc_intro
                          (MkSplit
                            a
                            (regex_lang a (deriv a d x r1))
                            (regex_lang a (Star a r1))
                            w
                            u2
                            (list_concat a rest)
                            (cong
                              (List a)
                              (List a)
                              (Cons a y (list_append a u2 (list_concat a rest)))
                              (Cons a x w)
                              (list_tail a)
                              e)
                            (ih1
                              u2
                              (transport_any
                                a
                                (λz. regex_lang a r1 (Cons a z u2))
                                y
                                x
                                (cong
                                  (List a)
                                  a
                                  (Cons a y (list_append a u2 (list_concat a rest)))
                                  (Cons a x w)
                                  (list_head_or a y)
                                  e)
                                p))
                            (trunc_intro
                              (MkPieces
                                a
                                (regex_lang a r1)
                                (list_concat a rest)
                                rest
                                Refl
                                allrest)))
                    })
                  allw
      }
  }

pub theorem deriv_complete
      (a : Type) (d : DecEq a) (x : a) (r : Regex a)
    : (w : List a) → regex_lang a r (Cons a x w) → regex_lang a (deriv a d x r) w =
  match r {
    Fail ↦ λw. λh. absurd h;
    Eps ↦ λw. λh. absurd h;
    Sym c ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (deriv a d x (Sym a c)) w)
            (λb.
              match b {
                Both exc nil ↦ guard_intro a (d.eq x c) (Eps a) w (d.complete x c exc) nil
              })
            h;
    Alt r1 r2 ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (deriv a d x (Alt a r1 r2)) w)
            (λo.
              match o {
                Inl h1 ↦
                  trunc_intro
                    (Inl
                      (regex_lang a (deriv a d x r1) w)
                      (regex_lang a (deriv a d x r2) w)
                      (deriv_complete a d x r1 w h1));
                Inr h2 ↦
                  trunc_intro
                    (Inr
                      (regex_lang a (deriv a d x r1) w)
                      (regex_lang a (deriv a d x r2) w)
                      (deriv_complete a d x r2 w h2))
              })
            h;
    Cat r1 r2 ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (deriv a d x (Cat a r1 r2)) w)
            (λs.
              match s {
                MkSplit u v e p q ↦
                  cat_split
                    a
                    d
                    x
                    r1
                    r2
                    (deriv_complete a d x r1)
                    (deriv_complete a d x r2)
                    w
                    u
                    v
                    e
                    p
                    q
              })
            h;
    Star r1 ↦
      λw.
        λh.
          elim_trunc
            (regex_lang a (deriv a d x (Star a r1)) w)
            (λps.
              match ps {
                MkPieces ws ec allw ↦ star_split a d x r1 (deriv_complete a d x r1) ws w ec allw
              })
            h
  }

theorem matches_sound_from
      (a : Type) (d : DecEq a) (w : List a)
    : (r : Regex a) → Equal Bool (regex_matches a d r w) True → regex_lang a r w =
  match w {
    Nil ↦ λr. λh. nullable_sound a r h;
    Cons x rest ↦
      λr. λh. deriv_sound a d x r rest (matches_sound_from a d rest (deriv a d x r) h)
  }

theorem matches_complete_from
      (a : Type) (d : DecEq a) (w : List a)
    : (r : Regex a) → regex_lang a r w → Equal Bool (regex_matches a d r w) True =
  match w {
    Nil ↦ λr. λh. nullable_complete a r h;
    Cons x rest ↦
      λr. λh. matches_complete_from a d rest (deriv a d x r) (deriv_complete a d x r rest h)
  }
```

## 5. Design notes

The denotation does not call `regex_matches` or `deriv`: its `Split` and
`Pieces` evidence describes real word factorisations. Truncation hides those
witnesses only when turning the existential reading into an Ω proposition.
The matcher decides one finite word with an explicit `DecEq a`; it does not
claim a finite quotient of generated derivatives or construct an automaton.

## 6. References

- [Regular expression](https://en.wikipedia.org/wiki/Regular_expression) —
  Wikipedia; orientation to language constructors and matching.
- [Brzozowski derivative](https://en.wikipedia.org/wiki/Brzozowski_derivative)
  — Wikipedia; orientation to left quotients and derivative matching.
- *Introduction to Automata Theory, Languages, and Computation* — Hopcroft,
  Motwani, and Ullman; background on regular languages and finite words.
- [Formal languages §4](../../../../spec/50-stdlib/61-formal-languages.md)
  — the public denotation, matcher, and six-law contract.

## 7. Trust & derivation

The public surface consists of `Regex` and `Fail`/`Eps`/`Sym`/`Alt`/`Cat`/`Star`,
`Split`/`MkSplit`, `Pieces`/`MkPieces`, `regex_lang`, `nullable`, `deriv`,
`regex_matches`, `nullable_sound`, `nullable_complete`, `deriv_sound`,
`deriv_complete`, `regex_matches_sound`, and `regex_matches_complete`.
The helpers `guard`, `cat_split`, `star_split`, the transport steps, and the
word-induction steps stay private.

| Reader task | Section |
|---|---|
| Describe a language | [Definition](#2-definition), [Using it](#3-using-it) |
| Match a word | [Definition](#2-definition), [Using it](#3-using-it) |
| Review both proof directions | [Laws & proofs](#4-laws--proofs) |

The definitions derive from checked `List`, `Bool`, `Equal`, `DecEq`, `Or`,
`And`, and truncation, with shared list and Boolean operations imported
from `Derived` and `LawfulClasses`. All six public laws are transparent,
kernel-checked proof terms. There is no local Axiom, postulate, primitive,
foreign declaration or `trusted_base()` entry; inherited assumptions of
loaded providers are not erased. The conformance seed
[seed-regex](../../../../conformance/stdlib/formal-languages/seed-regex.md)
exercises the finite-word behavior and proof-bearing examples.
