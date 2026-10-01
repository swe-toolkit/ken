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

**Construction (Architect rulings `evt_2yrn09v8znqx2`, `evt_734gcwk4m9f46`,
`evt_1wqksbwbp0q2z`, `evt_q0h185p9bckb`).**
- **Root dispatch.** `infer_match` dispatches on `!ind.indices.is_empty()`.
  Methods close by applying the unpeeled suffix. A failed `lower_by` gives
  `InferredMatchResultEscapesPattern`, naming the leaf-local binder (`m`).
- **One Elim builder.** `matrix_family_elim` splits the family's arguments
  at `ind.params.len()`. The nested split and the root constant-motive
  branch both use it, and the fixed Bool and List sites
  `debug_assert!(ind.indices.is_empty())`.
- **One motive authority.** The indexed root motive is built once, when
  `ret_ty_slot` is set, and it also feeds `finish_inferred_indexed_match`.
- **Nested splits revert dependent hypotheses** (McBride's construction).
  At a split on `x : D params is`:
  - Δ is the in-scope binders the nested methods consume whose type
    mentions `x` or its index variables, closed transitively and kept in
    context order. Membership is computed by free-variable occurrence in
    the current telescope.
  - The nested motive is `λ is. λ x'. Π Δ[x := x']. R`, built with
    `matrix_family_elim`.
  - Each method's expected type is
    `ken_kernel::inductive::method_type` over the nested Elim's own
    inputs. Its leaf compiles under fields, IHs and the specialized Δ', and
    it closes in check mode. No nested method or IH domain is built by
    hand.
  - The nested Elim is applied at the split site to the original Δ.
  - The motive is constant iff Δ = ∅ and `R` does not mention `x`. Then the
    term is byte-identical to today's constant-motive path.
  - **Index clause.** The nested target's indices must be distinct
    variables, occurring elsewhere only through Δ. Otherwise refuse with an
    existing non-`Internal`, non-`KernelRejected` `ElabError` that names the
    split. If no existing variant fits, stop to the Architect. This WP does
    no equational generalization.

## Acceptance

- **AC-0 (check 4; no build).** Write the obligation for a complete match
  through `build_checked_dependent_motive` with no omitted constructor. Say
  whether a nested-split method can be closed over the kernel's
  `method_type` domains. Say whether an unlowerable result is a surface
  error or a motive over the fields. The Architect rules before any edit.
- **AC-0: ruled** (`evt_2yrn09v8znqx2`).
- **AC-1.**
  - The three complete-match repros and the nested root split check, and
    the nested one passes the kernel recheck.
  - The field-dependent repro gives `InferredMatchResultEscapesPattern`
    naming `m`, and its annotated twin checks.
  - A Δ ≠ ∅ row with VNil/VCons specialization (the stop-3 shape) checks.
  - A Δ = ∅ control is byte-identical to the constant-motive path.
  - An index-clause refusal row is included if the clause is reachable on
    these fixtures. Measure which branch the nested repro takes.
  - The landed `lang_infer_match_index_coverage.rs` rows keep their
    results.
  - Name resolution across a split (`evt_1rhn5vnrq8egf`): a variable bound
    on a split column is referenced in the arm body, and a later field is
    referenced past the split. One row for a non-indexed family and one for
    an indexed family, run in a debug build.
  - Each mutation reddens only its own row:
    - reverting the dispatch reddens the complete-match rows;
    - reverting the suffix application brings back `Internal`;
    - reverting the nested Elim builder brings back `BadEliminator`;
    - hand-carried IH domains in place of `method_type` bring back the
      stop-3 `TypeMismatch`;
    - a lowerable control stays accepted.
- **AC-2 (controls).**
  - A genuinely reachable missing constructor still raises
    `ExhaustivenessError`, and so does a nested-column omission.
  - The 61-file catalog census is byte-identical.
  - Debug-profile gates (`evt_1rhn5vnrq8egf`; the census is release-built
    and cannot see a `debug_assert`): `ken-cli --test rt_parity_native`,
    every `lang_match_*` target, the `match_matrix_occurrence_tests` lib
    module, and the targets behind CI shards 1, 4, 5, 6 and 7, each named
    with its result.
- **Column visibility stays uniform** (`evt_1rhn5vnrq8egf`). A column's
  `surface_binder` is one value for every row (`elab.rs:17993`). A row whose
  own pattern does not source-bind a surface-visible column records that
  position on its `RowState` and hides it at the leaf, read from the
  occurrence's `source_binding` flag. Changing a column-level flag is a stop.

## Stop conditions

- Any kernel or trust change.
- A second index-impossibility rule beside the dependent path's.
- Accepting nested-column omission. That stays a residual of
  `LANG-INFER-MATCH-INDEX-COVERAGE`.

## Hard-stop inventory (§1b)

§1a count: 5 (Architect `evt_2t2kabhh5rhgn`, `evt_1rhn5vnrq8egf`,
`evt_7adqqrt4k6ye9`; research `evt_4m927ydg0rd7z`). The 6th stop triggers
research and the §1b predicate check.

1. The nested matrix split builds `Elim` with the indices inside `params`
   and `indices: []` (keyed on the construction site instead of the
   family's index arity).
2. The matrix compiler types an IH column by the inferred result `R`, while
   `method_type` types it as `motive(field indices, field)` (keyed on who
   types the IH column).
3. Nested method domains carry `root_motive m tl` built under the nested
   motive's binder and reuse it under the constructor-field telescope.

4. A per-row source-slot difference (a variable row binds the split value,
   a constructor row binds its fields) was expressed as the column-level
   `surface_binder`, which the all-flat push requires to be uniform (keyed
   on the column instead of the row).
5. A source variable bound at a split column has no `cx.ctx` binder, so
   positional resolution has nothing to return (keyed on resolving a name by
   context position when its value is a matrix occurrence). Ruled: it is an
   as-pattern over the row's constructor and resolves through the alias
   frame (`evt_7adqqrt4k6ye9`).

Entries 1-3 shared predicate: the nested split hand-assembles a piece of an
eliminator that the kernel re-derives from that eliminator's own inputs. Closed by
reverting dependent hypotheses (`evt_q0h185p9bckb`).
