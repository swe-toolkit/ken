---
id: LANG-NAMED-REFINEMENT-TYPE-ARGUMENT
title: "A conversion that holds only by unfolding a named refinement to its carrier is accepted with zero obligations: List Five admits Cons Int six, List Char admits 55296, and so do Int → Five results, classes, aliases, data fields and expression-level type arguments. Refuse the nested introduction at the check choke point, keyed on polarity and the checked refinement identity"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION, LANG-MATCH-RESULT-REFINEMENT-IDENTITY]
blocks: []
github: null
origin: "F-E: separate finding the Architect's TYPE-POSITION D0 ruling evt_5am0p8wy7vc9j asked to be measured; measured on c49297983 by the language implementer (evt_2tgrvgpr76kwm), boundary confirmed by the Architect (evt_19x6v93jytdy4). Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# A conversion that unfolds a named refinement where nothing introduces it is refused

## Objective

A conversion that holds only by unfolding a named refinement, at a position
where nothing introduces its predicate, is refused. A refinement whose value
really has the refined type, such as the prelude's `List Char`, stays legal.

## Settled inputs (Architect ruling `evt_2f9avjknr1xga`, on `f5d6d7f54`)

- **Withdrawn.** The position guard of `evt_7zj4g45k4t6dn` (rules 1 to 4,
  including the data gate) refused the prelude's `Char` under `List` and
  `Option` (stop 1, `evt_5r80vzfggwmn4`). WIPs `e139c06c6` and `43c293d98`
  stay as refs and are not candidates. `Some Char n` and `None Char` stay
  legal.
- **The leak is the conversion, not the type.** `Cons Int six (Nil Int)`
  checked against `List Five` is admitted by the kernel re-check's
  δ-conversion of `Five` to `Int`; `unify_types` solves level metas only
  (`evt_50b544fv42ney`). The prelude's `Char` has the same forgery:
  `const cs : List Char = Cons Int 55296 (Nil Int)` is accepted with 0
  obligations, while `const c : Char = 55296` emits 1
  (`evt_61vhrwkc583y3`).
- **The rule, from the D0' census `evt_1t4qxd37d8a06`.** A nested
  introduction is safe only at a function input (an odd count of `dom` steps)
  reached through no type argument or `Σ`, or at an equality's carrier.
  Equality endpoints are not exempt. Every legitimate row (`Ord.Char`,
  `DecEq.Char`, `RefinementView`, letter-frequency) is one of those; every
  forgery is an output position or under a type argument.
- **Mechanism, as ruled.** The census walk becomes a production guard,
  `nested_introduction_is_safe` over `ConvStep { Dom, Cod, Arg, Sig,
  EqCarrier }`. It δ-steps non-refinement `Const` heads so `Equal Char x y`
  becomes `Term::Eq` structurally. It runs first in
  `emit_refinement_introduction`, so its seven non-test callers are covered
  by construction, plus the transported type at the transported return. The
  top-level case stays `emit_refinement_introduction`'s.
- `forgets` (F-H) is out of scope: `LANG-NAMED-REFINED-BINDER-FIRST-CLASS`.
- **Held on `LANG-MATCH-RESULT-REFINEMENT-IDENTITY`** (stop 2,
  `evt_6qq1wtsekh42j`). The emit guard stays as built at WIP `398fda461`,
  with its 8 passing pins. The match routes drop the result refinement even
  at the top level, so the two match rows pass only after that WP lands.
  Rebase onto it, then finish.
- Kernel soundness is intact: the kernel sees the carrier.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The ruled guard, checked against the classified fan-in table in
`evt_50b544fv42ney`. The `lang_named` WIP tests that pinned the position rules are
replaced by the rows below.

## Acceptance

Base `f5d6d7f54` versus candidate, every row.

- **AC-1 (refused; base accepts with 0 open).** The `List Five` row and its
  `5` twin; `probe`'s argument; `Option Five`; `record Bag { xs = Cons Int
  six .. }`; P1, P2, P3's tail `t`, P8, P9, P10, P11, P12; `List (G Five)`
  with `const G : Type → Type = λa. List a` and value `Nil (List Int)`
  (`evt_4qbync1qst672`); forged `List Char` (`Cons Int 55296 (Nil Int)`) and
  forged `Option Char`. Also an indexed-match branch variable of local type
  `List Int` checked at `List Five`, and a non-dependent `match` whose arms
  infer `List Int` at expected `List Five` (pc and pd of stop 2): both
  refused once `LANG-MATCH-RESULT-REFINEMENT-IDENTITY` has landed.
- **AC-2 (controls; accepted, obligation counts unchanged).**
  - `ElabEnv::new()` and `intToChar`; `LawfulClasses` `Ord.Char` and
    `DecEq.Char`; Derived's `RefinementView`;
    `examples/rosetta/letter-frequency`.
  - P13 gives 1; `use5 six` gives 1; `record Box { value = six }` gives 1;
    `Cons Five five (Nil Five)` gives 1; `const c : Char = 55296` gives 1;
    `Some Char n` under `inRangeBool` is unchanged.
  - The 81 roots match the `f5d6d7f54` baseline (76 accepted, 32 open) file
    by file, and all 55 `catalog/packages` files exit 0.
- **AC-3 (mutation, QA).**
  - M-arg (treat `Arg` as safe) re-admits `List Five`.
  - M-polarity (treat an even `dom` count as safe) re-admits P1.
  - M-carrier (drop the `EqCarrier` exemption) refuses `LawfulClasses`.
  - M-transport (skip the guard on the transported return) needs a reaching
    row, or records "not expressible" with the attempt.
  - M-emit (guard only after the two inferred chokes) reddens the
    variable-route row, else the inferred-match row; if neither is
    expressible, say so.

## Stop conditions

- Any newly refused corpus, catalog, prelude or example program: stop with
  the row. Do not respell.
- An unaligned pair whose expected side still contains a refinement-rooted
  `Const` after the δ-steps, across the prelude, the 81 roots and the 55
  packages.
- Any kernel, `trusted_base()` or spec change.

## Symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. position-keyed refusal of a named refinement (type argument, expression, codomain) refused the prelude's `Char` under `List`/`Option`; keyed on syntactic position
2. nested-introduction guard keyed on elaborator check routes; match compilation checks leaves against an inferred or δ-simplified substitute, so the refinement never reaches `check` — keyed on the type a leaf is checked against
```
