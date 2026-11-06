---
id: LANG-EFFECT-ROW-CELL-SCRUTINEE-CARRIED-ROW
title: "A space cell whose type carries an effect row releases nothing to a match binder when it is the scrutinee directly or sits under an operand-joining form: inside `space Holder { mut held : Box = MkBox action ... }`, `match held { MkBox f ↦ let ignored = f MkUnit in MkUnit }` and `let p = (held, MkUnit)` then `match p { (MkBox f, _) ↦ ... }` both check and discard the callback, while `let b = held` and `let b : Box = held` refuse with E0038. RCell reads as pure on the evaluation row, and the checked type's carried row is joined only at a let binder with a named head. Give a cell its type's carried row on the latent plane, and pin both forms"
status: merged
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-EFFECT-ROW-LET-BOUND-SCRUTINEE-LATENT]
blocks: [LANG-PROC-NON-TREE-RESULT-TREE]
github: null
origin: "Architect AC-0 evt_2ga9m0e1ewqkk on LANG-EFFECT-ROW-LET-BOUND-SCRUTINEE-LATENT routed fixture q16 to the Steward as a sibling (relayed evt_50zmfe25kty5g); Verify QA's pair-cell finding evt_1xtw8a05xphx9 was folded in by Steward ruling evt_47wtcjarfvqde, confirmed by the Architect evt_1xzsnhrfrawtt. Steward-filed per COORDINATION section 2, sequenced after LET-BOUND-SCRUTINEE on the Verify ring."
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
text, as q17 (`let b : Box = held`) and q15 (`let b = held`) already
do. So does the same cell under a pair, Verify QA's probe
(`/tmp/qa-lbl-7f555-pair-cell.ken`, SHA256 `b9b4bbdc…bc8868`): bind
`p = (held, MkUnit)`, match `p` as `(MkBox f, _)`, and bind
`ignored = f MkUnit`.

## Settled inputs (main `22d1830e0`, LET-BOUND-SCRUTINEE landed)

- **The path.** In `crates/ken-elaborator/src/elab.rs`,
  `enter_match_latent` (`:1322`) gives a match's binders
  `infer_expr_row_type(scrut)` joined with
  `bound_value_latent_from_row`. In `infer_expr_row_type` (`:18039`),
  the arm at `:18058` reads RCell and RRecursiveResult, along with
  universes and literals, as the empty row. RPair joins its operand
  rows (`:18106`), and `bound_value_latent`'s default arm reuses that
  row, so `(held, MkUnit)` is empty joined with empty.
- **The let-binder rule.** `let_latent` (`:17997`) joins the carried row
  of the binder's checked type (`:18017`). That runs only at let
  binders, and only when the type has a named head, so a pair's Sigma
  gets nothing. LET-BOUND's D0 measured q15 refusing under it and q16
  still checking (evt_4sgz06h4twqzf). QA measured the pair-cell probe
  accepted at the pre-LET-BOUND base and at the candidate
  (evt_42tq2b4qd1hep).
- **The type is at hand.** `infer_space_cell` (`:14700`) types the cell.
  `written_type_row` (`:17774`) reads the same `carried_ids` table by
  GlobalId (`:17788`).
- **The plane (Architect evt_1xzsnhrfrawtt).** `infer_expr_row_type` is
  the evaluation row, and reading a cell performs nothing. The cell's
  carried row is a latent. Putting it on the evaluation row at `:18058`
  would charge it wherever a cell is read, including a sequenced tree's
  already-paid row, which LET-BOUND's sequenced branch leaves out.
- **Stated gap to correct.** The landed test header
  `lang_effect_row_let_bound_scrutinee_latent.rs` claims "other value
  forms", while its GAP names only the direct cell q16. This WP corrects
  the GAP to "RCell directly or nested under an operand-joining form".

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A match scrutinee's latent includes its checked type's carried row, for
every scrutinee form, or the scrutinee is refused at check time. One
candidate, `/crates` only.

A binder's carried row is read from its checked type as written, never
reduced, through the registered identities it names. Every fn, const
and top-level let is registered at admission with its body's written
footprint, an over-approximation: the join of every carried row its body
names, read unreduced. Only a non-empty footprint is inserted, and a
mutual group takes the join over its members (`elab.rs`
`declared_type_carried_row`, `modules.rs` mutual registration). No
declaration is classified as type-valued, because a result type need not
be written as a universe (`: AliasType`; Architect evt_5ja3eww6bq4gd). Reducing a binder's type is excluded: on CI it
cost the type-position guards 3 to 20 seconds, and it lost the alias
row that k11 carries (Architect evt_11pz35xvkdqgr, Research
evt_4gw4te9hrn1v9).

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the patch, and the named-binary commands with a bounded timeout.
  The Verify implementer runs it on scratch and reports. The Architect
  rules from that report, and does not build or test. It rules:
  - the plane, and where the carried row joins. The Architect's
    expectation is the latent plane: `bound_value_latent` recursing
    through the operand-joining forms, with RCell reading its cell
    type's carried row. The match and let paths read one rule;
  - every form through which a cell reaches a binder, enumerated from
    the `RExpr` variants: RCell and RRecursiveResult directly, and under
    RPair, RRecord, RBinOp, RStandardOp, RInfixSpine and the other
    operand joins;
  - that no row is paid twice: q11 (a sequenced tree) still prints `b`,
    and q02 (a pure callback) still prints `b`.
- **AC-1.** q16 and the pair-cell probe refuse with the E0038 "cannot
  be sequenced (36 §2.4)" text, and q15 and q17 still refuse the same
  way. Two same-form controls check: a pure callback in the cell
  (`MkPBox quiet`), and the constructor pair `(MkBox action, MkUnit)`.
  q18, the direct `match MkBox action`, still prints `ab`.
- **AC-2.** RRecursiveResult and each operand-joining form AC-0
  enumerated have a row, measured or refused, as ruled. The landed
  test's GAP line is corrected. LET-BOUND-SCRUTINEE's 11 pinned rows,
  the 13 MATCH-BINDER rows, the strict census (98, 64, 12, 52), frame
  budget row M on its unchanged stack, `surf_space_cells_p1`,
  `class_field_purity` and `purity_keywords` stay green.
- **AC-3, cost guards.** Every elaborator cost guard on the check path
  is a named gate: `elab_recursive_frame_budget`,
  `lang_branch_goal_contraction_cost`,
  `lang_branch_goal_delta_duplication`,
  `lang_type_position_sort_congruence_first` and
  `lang_type_position_sort_nested_domain`. The sweep key is any
  `crates/ken-elaborator/tests` file that asserts on elapsed time or
  frame size. k11 (an arrow alias), c13 (a lambda binder typed by a
  type-valued fn) and c14 (its let form) refuse with E0038. So do c15 to
  c18: a type function or const whose result type is an alias
  (`PickAlias On`, `AliasBox`); its pure control k16 prints `b`. Deleting
  the registration arm makes c15 to c18 check again.
- **Open sibling, not pinned.** k11i, a lambda binder whose latent arrow
  is written inline rather than through an alias, still drops its action.

## Stop conditions

- A kernel, trust or surface change.
- A form that can carry its row only by changing a spec 36 §1 rule.

Checks: 2 fired: the `:18058` arm is the fan-in for a cell however it
reaches a binder, directly or through an operand join, so AC-0
enumerates the forms from the `RExpr` variants, not from q16 and one
probe. 5 fired: the carried row is a latent, not an evaluation row, so
the plane is named in AC-0 and q11 guards against paying twice. 7 fired:
LET-BOUND's carried-row criterion, run on the direct and pair siblings,
fails at both; AC-2 runs it on every enumerated form. 8 fired: the
pure-callback cell and the constructor pair each share their probe's
form and check.
