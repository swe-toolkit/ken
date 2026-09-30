---
id: LANG-INFER-MATCH-INDEXED-COMPLETE
title: "An unannotated match on an indexed family elaborates only when it omits an arm: a complete match takes the non-indexed path and the kernel rejects it, and a root omission over a nested split reaches an Internal error. Every inferred match on an indexed family takes the one indexed path"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-INFER-MATCH-INDEX-COVERAGE]
blocks: []
github: null
origin: "Adversary M8 findings evt_7sdejvm46r9hw on ba2cd314c (LANG-INFER-MATCH-INDEX-COVERAGE): elaborator correctness and diagnostic gaps, no soundness issue (the kernel rejects every bad term). Steward-filed per COORDINATION section 2."
---

# One indexed path for inferred matches

## Objective

An unannotated `match` on an indexed family elaborates whether or not it
lists every constructor. Well-formed source never reaches
`ElabError::Internal` or a kernel de Bruijn error.

## Settled inputs (Adversary `evt_7sdejvm46r9hw`, read at `ba2cd314c`)

The repros use the landed test's `Vec` and a `Fin` with `FZ` and `FS`,
through `ElabEnv::elaborate_file`.
- **Complete matches are rejected.**
  - Repro: `fn c (a : Type) (n : Nat) (xs : Vec a n) : Nat = let r = match
    xs { VNil ↦ Zero; VCons m _ _ ↦ m } in r` gives `BadEliminator
    ("expected 1 params, got 2")`.
  - The same happens at `Vec a (Suc n)`. `Fin (Suc n)` with both arms
    gives `"expected 0 params, got 1"`.
  - `match xs { VCons m _ _ ↦ m }` at `Vec a (Suc n)` checks, so adding a
    lawful arm turns an accepted program into a kernel error. The checked
    (no-`let`) form is fine. The parent `a0c194ee8` failed the same way.
  - The cause: `index_coverage` (`elab.rs:18313`) fires only when a
    constructor row is missing. A complete match falls to the
    constant-motive path, which builds the `Elim` at `:18409-18416` with the
    indices inside `params` and `indices: vec![]`. The gate dispatches on
    "row missing", but the defect is on the "family is indexed" axis.
- **A root omission over a nested split reaches Internal.**
  - Repro: `let r = match xs { VCons m x VNil ↦ Zero; VCons m x _ ↦ m } in
    r` at `Vec a (Suc n)` gives `Internal("matrix method lost a constructor
    field/IH binder")`.
  - The cause: `close_inferred_index_method` (`:17679`) assumes the raw
    method is exactly `field_count + ih_count` lambdas, and a nested
    split's method is not shaped that way.
  - The parent gave `ExhaustivenessError VNil`.
- **A field-dependent result leaks.**
  - Repro: `... : Vec a n = let r = match xs { VCons m x tl ↦ tl } in r`
    gives `KernelRejected VarOutOfScope { index: 8, depth: 6 }`.
  - The cause: `lower_by(..).unwrap_or(zonked)` at `:17222` keeps the
    leaf-depth type when lowering fails.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Every inferred match on an indexed family takes the indexed path landed in
`ba2cd314c`, with `synthesize_omitted_index_method` still the only
impossibility authority. A nested split under the root and a result type
that cannot be lowered each either check or give a surface diagnostic.

## Acceptance

- **AC-0 (check 4; no build).** Write the obligation for a complete match
  through `build_checked_dependent_motive` with no omitted constructor. Say
  whether a nested-split method can be closed over the kernel's
  `method_type` domains. Say whether an unlowerable result is a surface
  error or a motive over the fields. The Architect rules before any edit.
- **AC-1.** Each of the five repros above checks, or gives a named
  surface `ElabError` that the Architect ruled for it. The landed
  `lang_infer_match_index_coverage.rs` rows keep their results.
- **AC-2 (controls).**
  - A genuinely reachable missing constructor still raises
    `ExhaustivenessError`.
  - The 61-file catalog census is byte-identical.

## Stop conditions

- Any kernel or trust change.
- A second index-impossibility rule beside the dependent path's.
- Accepting nested-column omission. That stays a residual of
  `LANG-INFER-MATCH-INDEX-COVERAGE`.
