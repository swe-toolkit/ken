# `Compare` — structural three-way comparison combinators

These comparison combinators depend only on the canonical `OrdResult` type and
structural `Pair` and `List` data. They sit below collection packages and lawful
class instances so each consumer reuses the same checked provider identities.

## Definition

```ken
import Core.Logic.Or (Or, Inl, Inr)
import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt)
import Core.Logic.Transport (sym)

pub fn pair_compare
      (a : Type)
      (b : Type)
      (cmpa : a → a → OrdResult)
      (cmpb : b → b → OrdResult)
      (x : Pair a b)
      (y : Pair a b)
    : OrdResult =
  match cmpa (pair_fst a b x) (pair_fst a b y) {
    Lt ↦ Lt;
    Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
    Gt ↦ Gt
  }

pub fn pair_compare_result_of (tail : OrdResult) (head : OrdResult) : OrdResult =
  match head {
    Lt ↦ Lt;
    Eq ↦ tail;
    Gt ↦ Gt
  }

pub proof eq for pair_compare
      (a : Type)
      (b : Type)
      (cmpa : a → a → OrdResult)
      (cmpb : b → b → OrdResult)
      (x : Pair a b)
      (y : Pair a b)
      (ha : Equal OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_eq)
      (hb : Equal OrdResult (cmpb (pair_snd a b x) (pair_snd a b y)) ord_eq)
    : Equal OrdResult (pair_compare a b cmpa cmpb x y) ord_eq =
  J
    (λr _.
      Equal
        OrdResult
        (match r {
          Lt ↦ Lt;
          Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
          Gt ↦ Gt
        })
        ord_eq)
    hb
    (sym OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_eq ha)

fn pair_compare_lt_cases_eq_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (peq : Equal OrdResult s ord_eq)
      (ptail : Equal OrdResult (cmpb sndx sndy) ord_lt)
    : Or
        (Equal OrdResult s ord_lt)
        (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_lt)) =
  Inr
    (Equal OrdResult s ord_lt)
    (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_lt))
    (and_intro (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_lt) peq ptail)

fn pair_compare_lt_cases_lt_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (plt : Equal OrdResult s ord_lt)
    : Or
        (Equal OrdResult s ord_lt)
        (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_lt)) =
  Inl
    (Equal OrdResult s ord_lt)
    (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_lt))
    plt

fn pair_compare_lt_cases_gt_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (pgt : Equal OrdResult s ord_gt)
      (plt : Equal OrdResult s ord_lt)
    : Or
        (Equal OrdResult s ord_lt)
        (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_lt)) =
  absurd (J (λr _. Equal OrdResult r ord_lt) plt pgt)

pub fn pair_compare_lt_cases
      (a : Type)
      (b : Type)
      (cmpa : a → a → OrdResult)
      (cmpb : b → b → OrdResult)
      (x : Pair a b)
      (y : Pair a b)
      (h : Equal OrdResult (pair_compare a b cmpa cmpb x y) ord_lt)
    : Or
        (Equal OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_lt)
        (And
          (Equal OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_eq)
          (Equal OrdResult (cmpb (pair_snd a b x) (pair_snd a b y)) ord_lt)) =
  match cmpa (pair_fst a b x) (pair_fst a b y) eqn : ha {
    Lt ↦ pair_compare_lt_cases_lt_at b cmpb (pair_snd a b x) (pair_snd a b y) Lt Proved;
    Eq ↦
      pair_compare_lt_cases_eq_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Eq
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_lt)
          h
          ha);
    Gt ↦
      pair_compare_lt_cases_gt_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Gt
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_lt)
          h
          ha)
  }

fn pair_compare_gt_cases_eq_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (peq : Equal OrdResult s ord_eq)
      (ptail : Equal OrdResult (cmpb sndx sndy) ord_gt)
    : Or
        (Equal OrdResult s ord_gt)
        (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_gt)) =
  Inr
    (Equal OrdResult s ord_gt)
    (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_gt))
    (and_intro (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_gt) peq ptail)

fn pair_compare_gt_cases_gt_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (pgt : Equal OrdResult s ord_gt)
    : Or
        (Equal OrdResult s ord_gt)
        (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_gt)) =
  Inl
    (Equal OrdResult s ord_gt)
    (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_gt))
    pgt

fn pair_compare_gt_cases_lt_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (plt : Equal OrdResult s ord_lt)
      (pgt : Equal OrdResult s ord_gt)
    : Or
        (Equal OrdResult s ord_gt)
        (And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_gt)) =
  absurd (J (λr _. Equal OrdResult r ord_gt) pgt plt)

fn pair_compare_gt_cases
      (a : Type)
      (b : Type)
      (cmpa : a → a → OrdResult)
      (cmpb : b → b → OrdResult)
      (x : Pair a b)
      (y : Pair a b)
      (h : Equal OrdResult (pair_compare a b cmpa cmpb x y) ord_gt)
    : Or
        (Equal OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_gt)
        (And
          (Equal OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_eq)
          (Equal OrdResult (cmpb (pair_snd a b x) (pair_snd a b y)) ord_gt)) =
  match cmpa (pair_fst a b x) (pair_fst a b y) eqn : ha {
    Lt ↦
      pair_compare_gt_cases_lt_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Lt
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_gt)
          h
          ha);
    Eq ↦
      pair_compare_gt_cases_eq_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Eq
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_gt)
          h
          ha);
    Gt ↦ pair_compare_gt_cases_gt_at b cmpb (pair_snd a b x) (pair_snd a b y) Gt Proved
  }

theorem pair_compare_eq_cases_eq_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (peq : Equal OrdResult s ord_eq)
      (ptail : Equal OrdResult (cmpb sndx sndy) ord_eq)
    : And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_eq) =
  and_intro (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_eq) peq ptail

theorem pair_compare_eq_cases_lt_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (plt : Equal OrdResult s ord_lt)
      (peq : Equal OrdResult s ord_eq)
    : And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_eq) =
  absurd (J (λr _. Equal OrdResult r ord_eq) peq plt)

theorem pair_compare_eq_cases_gt_at
      (b : Type)
      (cmpb : b → b → OrdResult)
      (sndx : b)
      (sndy : b)
      (s : OrdResult)
      (pgt : Equal OrdResult s ord_gt)
      (peq : Equal OrdResult s ord_eq)
    : And (Equal OrdResult s ord_eq) (Equal OrdResult (cmpb sndx sndy) ord_eq) =
  absurd (J (λr _. Equal OrdResult r ord_eq) peq pgt)

pub proof eq_cases for pair_compare
      (a : Type)
      (b : Type)
      (cmpa : a → a → OrdResult)
      (cmpb : b → b → OrdResult)
      (x : Pair a b)
      (y : Pair a b)
      (h : Equal OrdResult (pair_compare a b cmpa cmpb x y) ord_eq)
    : And
        (Equal OrdResult (cmpa (pair_fst a b x) (pair_fst a b y)) ord_eq)
        (Equal OrdResult (cmpb (pair_snd a b x) (pair_snd a b y)) ord_eq) =
  match cmpa (pair_fst a b x) (pair_fst a b y) eqn : ha {
    Lt ↦
      pair_compare_eq_cases_lt_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Lt
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_eq)
          h
          ha);
    Eq ↦
      pair_compare_eq_cases_eq_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Eq
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_eq)
          h
          ha);
    Gt ↦
      pair_compare_eq_cases_gt_at
        b
        cmpb
        (pair_snd a b x)
        (pair_snd a b y)
        Gt
        Proved
        (J
          (λr _.
            Equal
              OrdResult
              (match r {
                Lt ↦ Lt;
                Eq ↦ cmpb (pair_snd a b x) (pair_snd a b y);
                Gt ↦ Gt
              })
              ord_eq)
          h
          ha)
  }

pub fn list_eq (a : Type) (eqf : a → a → Bool) (xs : List a) (ys : List a) : Bool =
  match xs {
    Nil ↦
      match ys {
        Nil ↦ True;
        Cons h t ↦ False
      };
    Cons x xs2 ↦
      match ys {
        Nil ↦ False;
        Cons y ys2 ↦
          match eqf x y {
            True ↦ list_eq a eqf xs2 ys2;
            False ↦ False
          }
      }
  }

pub fn list_compare (a : Type) (cmp : a → a → OrdResult) (xs : List a) (ys : List a) : OrdResult =
  match xs {
    Nil ↦
      match ys {
        Nil ↦ Eq;
        Cons h t ↦ Lt
      };
    Cons x xs2 ↦
      match ys {
        Nil ↦ Gt;
        Cons y ys2 ↦
          match cmp x y {
            Eq ↦ list_compare a cmp xs2 ys2;
            Lt ↦ Lt;
            Gt ↦ Gt
          }
      }
  }
```

The attached proofs are declared with their defining `pair_compare` subject.
All declarations are ordinary checked Ken and add no trusted assumption.
