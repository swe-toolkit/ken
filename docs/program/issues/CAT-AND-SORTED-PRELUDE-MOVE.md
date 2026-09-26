---
id: CAT-AND-SORTED-PRELUDE-MOVE
title: "Move the prelude's And, and_intro, and_fst, and_snd and is_sorted into the L3 collections slice as checked catalog definitions, And spelled with expression-position Sigma and convertible with the prelude's, every catalog consumer importing them; the And-bearing successor to CAT-LOGIC-PRELUDE-MOVE"
status: ready
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

- **The prelude definitions** (`prelude.rs`):
  - `And` is registered at `:932`.
  - `and_intro`, `and_fst` and `and_snd` are the Σ intro/elim helpers at
    `:957-1015`.
  - `is_sorted` is at `:1266-1289` and uses `And` in its cons-cons case.
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
- **Home.** These names move together into the L3 collections slice, and
  there is no `Core.Logic.And` (`CAT-LOGIC-PRELUDE-MOVE` settled inputs).
  The Architect names the exact module at D0.
- **Known consumer.** `Algorithm.Searching.OrderedSearch` keeps its ambient
  `And`/`and_fst`/`and_snd` from `CAT-LOGIC-PRELUDE-MOVE`. It is the census's
  baseline-red residual (`lang_mod_strict_resolution_d0.rs`), so the census
  cannot see its names.
- **Out of scope.** The prelude registrations stay until the L2 flip; there
  is no `crates/` change except test expectations. `sort` and its obligation
  also stay out of scope unless D0 rules that they must move with
  `is_sorted`.

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
