---
id: LANG-NESTED-MATRIX-DERIVED-TELESCOPE
title: "The match matrix weaves split and IH binders that are not in the elaboration context while it builds, so every nested producer reconciles two coordinate systems by de Bruijn arithmetic, and a second split inside a bucket still fails with VarOutOfScope. Build each nested bucket inside the telescope its eliminator derives, with woven binders as real context pushes and the result type seeded or a metavariable"
status: active
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

- **Carry from the landed parent** (`7450a0b17`). The Architect's approval
  `evt_618n96gqvzeza` (`dec_1s48xec2d7m1x`) left a non-blocking census note
  on the guard direct-occurrence path. Read it at kickoff. The census here
  covers that path.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Three increments, each a straight-ancestor cut that may land alone.

0. **Fail closed on an outer alias inside an indexed inner match** (Adversary
   M8 on `7450a0b17`, `evt_1bspdnet7a87s`). Main gives silent wrong values
   when an outer `as` alias is read inside a match on an indexed family:
   - **R1**, a regression: base-rejected at `90ca730f6`, and now accepted
     with `f 1 (VCons Nat 1 5 (VCons Nat Zero 7 (VNil Nat))) 3 ⇝ 1`, where
     3 is expected;
   - **R2**, wrong on base too: `f Zero (VNil Nat) 3 ⇝ 2`, which is `j`.

   Both take the non-reverting path, where the in-matrix kernel check is
   gated off. One measured cause is `finalize_refined_body` (`elab.rs:7804`)
   shifting alias sentinels by `premise_count`. R1 has a second, unisolated
   mis-shift. **Ruled: refuse** (Architect `evt_4439x6wvc0m8v`, which
   carries the code). Both alias-frame pops (`finish_pattern_alias_frame`,
   `finish_pattern_alias_term_frame`) refuse with
   `PatternVariableAcrossDependentSplit` when an enclosing frame's sentinel
   survives in a match whose scrutinee family refines indices. Every caller
   passes the truth for its own scrutinee; a constant `None` says why.
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
   - The in-matrix alias finalize and per-method kernel check run in the
     derived telescope on every path. Both `needs_reverting` gates are
     removed (`evt_4c4tgkc2gvypk`, `evt_qh7m7erbc5f6`, input f).
   - Representation (input g, `evt_71d3p2fwxtqj1`): locally nameless. Matrix
     binders are elaborator-only fresh free variables, refused at the kernel
     boundary, so the kernel `Term` is unchanged. One `abstract` function
     does the index arithmetic, once, at closing.

## Acceptance

- **AC-0a (increment 0, lands alone on `7450a0b17`).**
  - All 11 indexed-family rows of `evt_1bspdnet7a87s` refuse with
    `PatternVariableAcrossDependentSplit`, each a refusal pin that names
    increment 2 as its flip to 3.
  - The finding's seven correct rows are pinned as values: outer alias in a
    tuple split, in a `List` nested-head split and in a flat `List` match;
    an outer nested-pattern variable; a plain outer variable in an indexed
    inner match. The FIELD-DEP value pins and AC-3's suites stay green. A
    committed test that newly refuses is a stop to the Architect.
  - Fan-in: every caller of the two pops and the flag it passes, and the
    path the checked R1 and R2 variants take. A check-mode alias frame
    closed elsewhere is a stop.
  - Census rows c1 (an indexed match's own alias) and c2 (an alias across a
    nested indexed-column split) are reported, not asserted. A wrong value
    is a stop to the Architect.
  - Forcing the flag to `None` in `infer_match` restores R1 ⇝ 1 and R2 ⇝ 2.
- **AC-1.** The M-deep Zero fixture and its two-field sibling flip from
  transition sentinel to their normalized values.
  The R1 and R2 rows flip from refusal to 3, built from occurrence terms in
  the derived telescope.
- **AC-2 (controls).**
  - Every value pin of `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` stays green.
    That includes the collision control and M1, M2 and M3.
  - A dependent-R fixture in inference mode gives the precise diagnostic.
  - A constant-R fixture in inference mode still checks.
  - The `lang_match_record_pattern` record row and the reverting
    woven-column fixture (FIELD-DEPENDENCE P5), the Dep row and the deferred
    wrong-method fixture are discriminating value pins, checked in-matrix.
  - `PatternVariableAcrossDependentSplit` is removed (input h).
- **AC-3.** `lang_infer_match_indexed_complete` and the as-pattern,
  nested-split and tuple-pattern suites stay green. The catalog census is
  byte-identical, and `trusted_base()` is unchanged.

## Stop conditions

- The repair needs a kernel or spec change.
- Seeding leaves an entry `None` in check mode: stop and name it.
- A retired mechanism has a consumer outside the matrix: stop and name it.
