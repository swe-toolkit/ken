---
id: LANG-MATCH-ARM-REFL-PAIR-GOAL
title: "Refl in the Nil arm of a match whose goal is Equal (Pair q r) ... is refused with 'Refl expects an Eq-shaped goal', while the identical goal stated as a standalone lemma accepts Refl. The arm's expected type is not reduced to an Eq before Refl is checked. Check Refl against the arm goal as the lemma does"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Catalog Finding from CAT-FORMAL-LANGUAGES-DFA (06-catalog-campaign.md Findings routing). Architect measured it on a 2026-10-07 binary (evt_7z0tswfa1wd9); the Foundation implementer reproduced it on e5ec530dc (evt_7f1fe7xrnjmn1). The package works around it with the run_product_nil lemma. Steward-filed per COORDINATION section 2."
---

# Refl checks in a match arm at a Pair carrier

## Objective

A match arm whose goal is an `Equal` between convertible terms accepts
`Refl`, exactly as the same goal does as a standalone declaration.

## Settled inputs (measured at `e5ec530dc`, `evt_7f1fe7xrnjmn1`)

- **The failing declaration.**
  ```ken
  theorem run_product (q : Type) (r : Type) (a : Type)
      (combine : Bool → Bool → Bool) (d : Dfa q a) (e : Dfa r a)
      (s1 : q) (s2 : r) (w : List a)
      : Equal (Pair q r)
          (run (Pair q r) a (product q r a combine d e) (mk_pair q r s1 s2) w)
          (mk_pair q r (run q a d s1 w) (run r a e s2 w)) =
    match w {
      Nil ↦ Refl;
      Cons x rest ↦ run_product q r a combine d e (step q a d s1 x)
        (step r a e s2 x) rest
    }
  ```
  It gives `TypeMismatch { reason: "Refl expects an `Eq`-shaped goal" }`
  at the `Refl` (rc=1). `Dfa`, `run`, `step`, `product` are in
  `catalog/packages/Algorithm/FormalLanguages/Dfa.ken.md`.
- **The control.** The `Nil` instance of that goal, as the standalone
  `theorem run_product_nil ... = Refl`, checks (Architect
  `evt_7z0tswfa1wd9`).
- The cause is not yet localized: the arm's motive-instantiated goal is
  not seen as `Eq`-shaped where `Refl` is checked.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** The arm's expected type at the `Refl` check, and
   whether a non-`Pair` carrier (`Equal Nat ...` in a list-match arm)
   fails the same way. The Architect rules the repair at D0.
2. **The ruled repair.** `Refl` in an arm is checked against the arm goal
   reduced as for a standalone declaration.

## Acceptance

- **AC-1.** The declaration above checks with `Nil ↦ Refl`. The package can
  then drop `run_product_nil`; that edit belongs to a later catalog change,
  not this WP.
- **AC-2 (control).** A `Refl` arm whose two sides are not convertible is
  still refused, with a not-convertible reason.
- **AC-3 (mutation, QA).** Reverting the repair reddens AC-1.

## Stop conditions

- The repair needs a kernel, trust or spec change.
