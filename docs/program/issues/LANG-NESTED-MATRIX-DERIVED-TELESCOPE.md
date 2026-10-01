---
id: LANG-NESTED-MATRIX-DERIVED-TELESCOPE
title: "The match matrix weaves split and IH binders that are not in the elaboration context while it builds, so every nested producer reconciles two coordinate systems by de Bruijn arithmetic, and a second split inside a bucket still fails with VarOutOfScope. Build each nested bucket inside the telescope its eliminator derives, with woven binders as real context pushes and the result type seeded or a metavariable"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-NESTED-SPLIT-FIELD-DEPENDENCE]
blocks: []
github: null
origin: "Architect evt_5eahknef2gfbm (Part 2) on the 6th advancing stop of LANG-NESTED-SPLIT-FIELD-DEPENDENCE (evt_6xyk9hsbe6h7m), with research advisories evt_4p5eqpfdywk97 and evt_b6d09v62nfmt. Steward disposition evt_7dwx0stmecwd8. Steward-filed per COORDINATION section 2."
---

# Nested matches built in their own telescope

## Objective

A split nested inside a bucket of another split elaborates against the
telescope its eliminator derives. No nested method domain, motive or alias
occurrence is computed by depth arithmetic on a term from another frame.

## Settled inputs (Architect `evt_5eahknef2gfbm`, read at WIP `9a5b617c5`)

- **The predicate.** `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` §1b entries 1, 2,
  3, 5 and 6 share it: a term in one frame's coordinates is used in another
  frame's context through depth arithmetic. The matrix weaves split x' and IH
  slots that are not in `cx.ctx` during construction.
- **Prior art** (research, read to understand only). GMM 2006, Lean 4
  `cases`/`induction`, Rocq `match … return` and Agda case trees all build
  each inner split top-down, inside the telescope the outer method type
  opens. The check-mode result type is seeded from the expected type.
- **The result type is never known up front today.** `ret_ty_slot` is `None`
  at all 24 measured nested-split entries (15 landed, 9 WP), including a
  `def` with a declared `PairOut` result. That is a wiring gap (logs
  `/workspaces/ken/tmp/field-dependence-m-r-{landed,wp}-suite.log`).
- **What it retires**: entry 1's tail rebase, the nested use of
  `indexed_root_ih_domain`, the `close_nested_matrix_method` re-wrap, and
  the enclosing-frame sentinel deferral.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Two increments, each a straight-ancestor cut that may land alone.

1. **Seed `ret_ty_slot` from the check-mode expected type.** Report how many
   of the 24 entries become `Some`.
2. **Open each nested bucket in the derived telescope before its leaves.**
   - Δ comes from reverting the context and the constructor. Woven binders
     are real `cx.ctx` pushes.
   - R is the seeded type, or else a fresh result metavariable solved by the
     first leaf. Constant results keep first-leaf discovery.
   - In pure inference mode, an annotation is required only when the solved
     R would mention a derived-telescope binder. The precise diagnostic is
     raised there; "the split reverts" is not the test.
   - Occurrence terms replace sentinel depth arithmetic, and the four
     mechanisms above are deleted.
   - The in-matrix per-method kernel check runs in the derived telescope on
     every path. The `&& needs_reverting` conjunct is removed
     (`evt_4c4tgkc2gvypk`, input f).

## Acceptance

- **AC-1.** The M-deep Zero fixture and its two-field sibling flip from
  transition sentinel to their normalized values.
- **AC-2 (controls).**
  - Every value pin of `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` stays green.
    That includes the collision control and M1, M2 and M3.
  - A dependent-R fixture in inference mode gives the precise diagnostic.
  - A constant-R fixture in inference mode still checks.
  - The `lang_match_record_pattern` record row and the reverting
    woven-column fixture (FIELD-DEPENDENCE P5) are value-pinned and checked
    in-matrix.
- **AC-3.** `lang_infer_match_indexed_complete` and the as-pattern,
  nested-split and tuple-pattern suites stay green. The catalog census is
  byte-identical, and `trusted_base()` is unchanged.

## Stop conditions

- The repair needs a kernel or spec change.
- Seeding leaves an entry `None` in check mode: stop and name it.
- A retired mechanism has a consumer outside the matrix: stop and name it.
