---
id: LANG-NAMED-REFINEMENT-TYPE-ARGUMENT
title: "A named refinement at a position where nothing introduces its predicate is accepted with zero obligations: List Five admits 6, and so do Int → Five parameters and results, classes, aliases, data fields and expression-level type arguments. The literal guards key on the spelling RRefine. Refuse the named form at every position where nothing introduces it, keyed on its checked refinement identity"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION]
blocks: []
github: null
origin: "F-E: separate finding the Architect's TYPE-POSITION D0 ruling evt_5am0p8wy7vc9j asked to be measured; measured on c49297983 by the language implementer (evt_2tgrvgpr76kwm), boundary confirmed by the Architect (evt_19x6v93jytdy4). Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# A named refinement where nothing introduces it is refused

## Objective

A named refinement at a position where nothing introduces its predicate is
refused, keyed on its checked refinement identity. It stays admitted where a
value is checked against it: a whole annotation, or a parameter of a written
function type.

## Settled inputs (Architect ruling `evt_7zj4g45k4t6dn`, on `6b938b70b`)

- **Route (b) is closed.** `MetaCtx` solves levels only, so no meta is ever
  solved to `Five`.
- **The predicate.** `Five` survives elaboration as a bare `Const` and is
  introduced only where a value is checked against that `Const`. Everywhere
  else, a value reaches the position by conversion through `Int`. The three
  literal guards key on the spelling `RRefine`, so the named form slips past
  all of them: the `elab_type` slot arm, `alias_nested_refinement` and
  `refine_return_depth`.
- **Measured leaks** (base: accepted, 0 obligations): `List Five`, an
  `Int → Five` parameter (P1), a curried `Five` result (P8), a class field
  (P9), an alias of `Int → Five` (P11), a data field `Mk (Int → Five)`
  (P12), and expression-level type arguments (P2, P3, P10). The P-rows are
  listed verbatim in the ruling.
- **Stays admitted.** `fn take (k : Five → Int)` (P13) and Derived's
  `RefinementView.project : TrueBool → Bool` introduce at application, so
  there is no catalog edit.
- **Mechanism, as ruled.**
  - A `NamedRefinementPosition` is carried like `refinement_slot`.
  - The `RCon` and `RCheckedGlobal` arms refuse a `refinement_root` id at an
    unintroduced position.
  - A named-result depth check runs on every `elab_signature` route that
    binds params.
  - The `infer` arms refuse a refinement-rooted `Const` in expression
    position.
  - The data gate routes refinement-rooted field, param and index types.
- Kernel soundness is intact: the kernel sees `Int`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The ruled closure, with the expression fan-in enumerated in the handoff.

## Acceptance

Base versus candidate, every row.

- **AC-1 (refused, naming the refinement and the place).** The `List Five`
  row and its `5` twin; `probe (xs : List Five)`; `Option Five`;
  `record Bag { xs : List Five }`; `def L = List Five2`, refused at `L`;
  `List (G Five)` with `def G (a : Type) = List a`; and P1, P8, P9, P11,
  P12, P2, P3, P10.
- **AC-2 (controls; obligation counts unchanged).**
  - `use5 six` gives 1; `record Box { value : Five }` gives 1;
    `fn r (n : Int) : Five = six` gives 1; P13 gives 1.
  - A `Five2` binder with `def Five2 = Five` gives 1.
  - A class field `get : Five` keeps its base count.
  - `ken check` on `Derived.ken.md` exits 0.
  - The 81-file corpus census matches base file by file.
  - The expression census covers every expression-position reference to a
    refinement identity across `catalog/`, `library/`, `examples/`,
    `crates/*/tests` and `conformance/`. Report how many `Equal Five ..`
    uses (p6) the rule refuses.
- **AC-3 (mutation, QA).** Each mutant restores its row to accepted:
  - M-arg on `List Five`;
  - M-codomain on P1;
  - M-depth on P8;
  - M-expr on P10;
  - M-gate on P12.

## Stop conditions

- Any newly refused corpus, catalog, library or example program, or any
  non-probe expression-position use: stop with the row. Do not respell.
- Any kernel, `trusted_base()` or spec change.
