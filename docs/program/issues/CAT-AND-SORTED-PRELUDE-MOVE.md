---
id: CAT-AND-SORTED-PRELUDE-MOVE
title: "Move the prelude's And, and_intro, and_fst, and_snd and is_sorted into the L3 collections slice as checked catalog definitions, And spelled with expression-position Sigma and convertible with the prelude's, every catalog consumer importing them; the And-bearing successor to CAT-LOGIC-PRELUDE-MOVE"
status: merged
owner: foundation
size: M
gate: architect
tier: T1
depends_on: [LANG-EXPRESSION-SIGMA]
blocks: []
github: null
origin: "Operator rulings 2026-09-25 (minimal fixed prelude; convenience names are technical debt moved to packages). CAT-LOGIC-PRELUDE-MOVE settled inputs put And/and_*/is_sorted out of that slice until LANG-EXPRESSION-SIGMA lands, to move together in the L3 collections slice, with no Core.Logic.And (Architect evt_1ymf3p94jygx3, evt_16ve7jwxbzax3). Steward-filed per COORDINATION section 2."
---

# Move And and is_sorted out of the prelude

## Objective

Every catalog module that uses `And`, `and_intro`, `and_fst`, `and_snd` or
`is_sorted` resolves it through an import of a catalog package, never
through the prelude fall-through.

## Settled inputs -- to re-measure at the base

- **The prelude definitions** (`prelude.rs` at `0ccd87dd6`):
  - `And` is registered at `:1304`.
  - `and_intro`, `and_fst` and `and_snd` are the Σ intro/elim helpers at
    `:1329-1387`.
  - `is_sorted` is at `:1477-1502` and uses `And` in its cons-cons case.
- **`And` needs expression-position Σ** (`LANG-EXPRESSION-SIGMA`). A `fn`
  head must be a lowercase `value_name` (32 §1), so the catalog declares
  `pub fn and (a : Omega) (b : Omega) : Omega = (x : a) × b` and republishes
  it with `export and as And`. A renamed export mints no new `GlobalId`, so
  importers' `And a b` is the same checked identity (Architect
  `evt_5ertv6rf0hefy`; Spec route (a), seed `seed-expression-sigma.md`).
  The body must be convertible with the prelude's `And`; the Sigma
  migration evidence (its AC-1a) already measures that conversion through
  the renamed export.
- **Carried pin.** Re-key `lang_expression_sigma.rs`'s `globals["And"]` pin
  to the catalog identity in this move (Architect carry).
- **Home (D0 `evt_56k6wn8p42efg`).**
  - **And family.** A new leaf module `Core.Logic.Conjunction` with no
    imports, so `Core.Logic.Compare` can import it. A collections home
    would be a cycle, because `Data.Collections.Derived` imports Compare.
  - **Helpers.** An Ω-valued `fn` is refused, so the three helpers are
    `pub theorem`.
  - **Cross-reference.** The module cross-references `Core.Logic.And`,
    the Type-sorted, proof-relevant `data And` that Nfa and Regex use; a
    rename of either is a carry.
  - **`is_sorted`.** It goes in `Data.Collections.Derived` §2, directly
    after `filter`.
  - **Code.** The D0 gives the declarations, and its measurements cover
    conversion, controls and loading.
- **OrderedSearch is an ordinary consumer.** It has a normal census row,
  and `expected_residuals` is empty, so it imports like the others.
- **Known consumer.** `Capability.System.Error` uses the ambient `And`,
  `and_intro`, `Prod` and `MkProd` (its strict-resolution census row, from
  `CAT-SYSTEM-ERROR-PRELUDE-MOVE`; Architect `evt_6qgpyh2yqeexa`).
- **Out of scope.** The prelude registrations stay until the L2 flip; there
  is no `crates/` change except test expectations. `sort` and its obligation
  also stay out of scope: no prelude consumer pins `is_sorted` (D0).
- **Two `is_sorted` identities until the flip.** The catalog and prelude
  `is_sorted` are not convertible on an open list, and a bare loose import
  is ambiguous. Every mix fails closed, so loose harnesses import it by
  alias, and pins are re-keyed by checked identity (the D0's list).
- **Carry to the L2 flip.** Deleting prelude `is_sorted` needs nothing in
  the prelude. Deleting `And` needs a disposition for the prelude's own
  `resource_settle_*` theorems, which consume `And` and `and_intro`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Checked catalog definitions of `And`, `and_intro`, `and_fst`, `and_snd` and
`is_sorted` with the prelude's statements, in the module D0 names. Every
catalog consumer imports them.

## Acceptance

- **AC-0 (D0, before the move).**
  - The Architect names the home module.
  - The Architect rules whether a prelude consumer of `is_sorted` (for
    example `sort`'s obligation) pins it to the prelude.
  - Post the consumer population, swept by mechanism across every source
    root: `catalog/`, `crates/*/tests`, `r_layer_tests`, `seal2_tests`,
    `examples/`, `conformance/`, CLI fixtures and formatter fixtures. The
    sweep includes export-surface pins and loose sources that name the bare
    spelling (`CAT-LOGIC-PRELUDE-MOVE` inventory line 2).
- **AC-1 (identity).**
  - The catalog `And` is convertible with the prelude `And` in both
    directions.
  - Each moved helper checks with the prelude's statement.
- **AC-2 (controls).**
  - Removing each consumer's import reddens that consumer.
  - The strict census moves only the rows AC-0 predicts.
  - The prelude's own checked definitions still elaborate.
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- Any kernel, `trusted_base()` or spec change, or a non-convertible `And`.
- A prelude definition that must keep a moved name, unless D0 rules it.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
