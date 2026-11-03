---
id: LANG-EFFECT-ROW-CELL-SCRUTINEE-CARRIED-ROW
title: "A match directly on a space cell whose type carries an effect row releases nothing to its binders: inside `space Holder { mut held : Box = MkBox action ... }`, `match held { MkBox f ↦ let ignored = f MkUnit in MkUnit }` checks, while the let-bound forms of the same scrutinee refuse with E0038. infer_expr_row_type reads RCell and RRecursiveResult as pure, and the checked type's carried row is joined only at let binders. Give a match scrutinee its checked type's carried row, and pin the direct cell scrutinee"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-EFFECT-ROW-LET-BOUND-SCRUTINEE-LATENT]
blocks: [LANG-PROC-NON-TREE-RESULT-TREE]
github: null
origin: "Architect AC-0 evt_2ga9m0e1ewqkk on LANG-EFFECT-ROW-LET-BOUND-SCRUTINEE-LATENT routed fixture q16 to the Steward as a sibling (relayed evt_50zmfe25kty5g). Steward-filed per COORDINATION section 2, sequenced after LET-BOUND-SCRUTINEE on the Verify ring."
---

# A cell scrutinee carries its type's row

## Objective

A checked program never accepts a match-bound callback whose action
cannot be sequenced. In the LET-BOUND-SCRUTINEE D0 fixtures
(`mk-fixtures.sh`, md5 `ef0c438586374dd272a3cbc24a98da9a`), q16:

```ken
space Holder {
  mut held : Box = MkBox action
  proc fire () : Unit visits [Holder, Console] =
    match held { MkBox f ↦ let ignored = f MkUnit in MkUnit }
}
```

refuses at check time with the E0038 "cannot be sequenced (36 §2.4)"
text, as q17 (`let b : Box = held`) and q15 (`let b = held`, once
LET-BOUND-SCRUTINEE lands) already do.

## Settled inputs (main `e6e13def6`)

- **The path.** In `crates/ken-elaborator/src/elab.rs`,
  `enter_match_latent` (`:1322`) gives a match's binders
  `infer_expr_row_type(scrut)` joined with `bound_value_latent(scrut)`.
  The arm at `:17994` reads RCell and RRecursiveResult, along with
  universes and literals, as the empty row.
- **The let-binder fix.** The ruled LET-BOUND-SCRUTINEE patch adds
  `let_latent`, which joins the carried row of the binder's checked type
  (`core_type_head_id` of the zonked whnf type, looked up in
  `carried_ids`). That is X3, and it runs only at let binders. Its D0
  measured q15 refusing under X3, and q16 still checking in all four arms
  (evt_4sgz06h4twqzf).
- **The type is at hand.** `infer_space_cell` (`:14699`) types the cell.
  The written-annotation route, `written_type_row` (`:17773`), reads the
  same `carried_ids` table by GlobalId (`:17787`).

Treat anchors as perishable, and re-read them on LET-BOUND-SCRUTINEE's
landed base. If a settled input is false there, stop and report the
mismatch; do not build around it.

## Deliverable

A match scrutinee's latent includes its checked type's carried row, for
every scrutinee form, or the scrutinee is refused at check time. One
candidate, `/crates` only.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the patch, and the named-binary commands with a bounded timeout.
  The Verify implementer runs it on scratch and reports. The Architect
  rules from that report, and does not build or test. It rules:
  - where the carried row joins: at `enter_match_latent`, or in a shared
    helper with `let_latent`, so that the let and match paths read one
    rule;
  - every scrutinee form whose infer row omits its type's carried row,
    enumerated from the `RExpr` variants. RCell and RRecursiveResult are
    the known two;
  - that the join does not pay a row twice when the scrutinee's infer
    row already contains it.
- **AC-1.** q16 refuses with the E0038 "cannot be sequenced (36 §2.4)"
  text, and q15 and q17 still refuse the same way. A same-form control
  with a pure callback in the cell (`MkPBox quiet`) checks. q18, the
  direct `match MkBox action`, still prints `ab`.
- **AC-2.** RRecursiveResult has a row, either measured or refused, as
  ruled. LET-BOUND-SCRUTINEE's pinned fixtures, the 13 MATCH-BINDER rows,
  the strict census (98, 64, 12, 52), frame budget row M on its unchanged
  stack, `surf_space_cells_p1`, `class_field_purity` and
  `purity_keywords` stay green.

## Stop conditions

- A kernel, trust or surface change.
- A form that can carry its row only by changing a spec 36 §1 rule.

Checks: 2 fired: the `:17994` arm is the fan-in for both cell and
recursive-result scrutinees, so AC-0 enumerates the forms from the
`RExpr` variants, not from q16 alone. 7 fired: LET-BOUND's X3 criterion,
run on the direct-match sibling, fails at q16; AC-2 runs it on every
scrutinee form. 8 fired: the pure-callback cell control has the same
space, cell and match form, and checks.
