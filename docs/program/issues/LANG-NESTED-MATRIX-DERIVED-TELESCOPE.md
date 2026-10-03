---
id: LANG-NESTED-MATRIX-DERIVED-TELESCOPE
title: "The match matrix weaves split and IH binders that are not in the elaboration context while it builds, so every nested producer reconciles two coordinate systems by de Bruijn arithmetic, and a second split inside a bucket still fails with VarOutOfScope. Build each nested bucket inside the telescope its eliminator derives, with woven binders as real context pushes and the result type seeded or discovered first"
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

Four increments (0, 1, 1b, 2), each a straight-ancestor cut that may land
alone.

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

   **The check path** (Architect `evt_6n6r4r5shx05f`, `evt_1220jbxcj19zq`,
   corrected by `evt_40f42ed7ayqq5`, which carries the code). A checked
   method body opens no alias frame. It reaches the same shift through
   `wrap_premise_lams_finalized` and
   `wrap_premise_pis_finalized` at `check_dependent_branch_body` `:6318` and
   `check_match_dependent_mode` `:6590`. A checked single-arm `Vec Nat (Suc
   n)` sibling normalizes to `Zero` where 3 is expected. That is measured on
   `a252de8d8` and on `origin/main` `e893ecb7a`, so it is a live miscompile on
   main, and increment 0 turns it into a refusal (`evt_4wqqh74aat58j`).
   - Both wraps return `Result<Term, AliasAcrossPremiseWrap>` and refuse
     whenever premises are non-empty and the body or a premise holds an alias
     sentinel. No wrap runs inside a live own frame: a finishing frame
     resolves its own sentinels before any wrap, so c1 is unaffected.
   - All ten call sites map the error to `PatternVariableAcrossDependentSplit`
     at the split's span, with no `.ok()`, `unwrap`, `expect` or `_` arm.
   - `wrap_premise_lams_from_full` (`:13536`) is excluded because it never
     calls `finalize_refined_body`.
   - The infer-finisher guards stay.

   `Internal(g0)` from `project_generated_index_equality_leaves` is not the
   refusal.
1. **Seed `ret_ty_slot` from the check-mode expected type** (merged
   `2d6b64aca`, exact `b16a22f1a`). Every
   check-mode split entry becomes `Some`. Inference-mode entries stay
   `None`, and increment 2 serves them. The handoff reports arrivals by
   population and mode (Steward `evt_vj9sdpww9tfe`). The 24-entry figure is
   superseded, since increment 0 refuses one historical entry.
   - With the seeding disabled, 18 of the 19 current check-mode entries
     return to `None`. The 19th gets R from an earlier root leaf, the
     slot's second producer, so it is not evidence for the seed.
   - A first-bucket unit pin, where no earlier leaf can supply R, reddens
     on seed-off (`evt_6ez9sxdhdkb8a`).
1b. **Seed only a goal whose levels are solved** (merged `710458002`, exact
   `7c57aac8d`; Architect `evt_6jqa2y7rft3tr`, on Adversary
   `evt_22n4tn23eqwjn`). On `2d6b64aca`, a
   check-mode `: Type` match whose arms are `Type` is `KernelRejected
   TypeMismatch` on the tuple, or-pattern and nested paths. All three were Ok
   at `a246ede23`. `zonk_level` (`elab.rs:137-140`) reads the unsolved `?u`
   as Zero, so the seed is `Type 0`. No leaf unifies against the slot, and
   the final `unify_types` solves `?u := 0`.
   - `MetaCtx` gains a `defaulted: Cell<bool>` witness, which the unsolved
     arm of `zonk_level` sets.
   - One helper, `check_mode_result_seed`, replaces all five seed sites:
     infer_tuple_match `:19140`, infer_record_match `:19223`, infer_or_match
     `:19370`, infer_literal_match `:19480` and infer_match `:19616`. It
     returns `None` when zonking defaulted a level, so the first leaf
     discovers R as before increment 1. The Architect's ruling carries the
     code.
2. **Open each nested bucket in the derived telescope before its leaves.**
   - Δ comes from reverting the context and the constructor. Woven binders
     are real `cx.ctx` pushes.
   - **R: infer, then check** (Architect `evt_9s1tts2m0xjk`; the earlier
     "fresh result metavariable" is withdrawn, and no term-meta layer is
     added). R is the seeded type. In inference mode, when the construction
     needs R first at an `Ih` column (`:18240-18255`) or a root IH domain, a
     discovery pass runs the same matrix code, with an `Ih` column pushing
     no binder. The first reachable leaf computes R as today (`zonk_term`,
     `lower_by`, `InferredMatchResultEscapesPattern`). The owning entry
     (keyed on `root_frame_depth`) then aborts with `ResultDiscovered` and
     reruns once with `ret_ty_slot = Some(R)`; a second discovery on the
     rerun is `Internal`. A first leaf reached before any IH keeps
     first-leaf discovery, with no rerun.
     - Reuse, not rollback (`evt_1bg60xdd88p6v` supersedes the restore
       set). `MetaCtx` and `GlobalEnv` are not restored or cloned. The
       discovery leaf is elaborated once through `compile_match_leaf`; its
       body, lowered R, arm, context length and skipped IH depths are
       cached, and the rerun's first leaf reuses the body through one
       checked thinning that inserts only the recorded IH binders. A
       mismatch is `Internal`. Assert the scoped stacks balanced at abort.
     - Descent writes before the first leaf (`evt_64e2sz53dmzrx`): only
       literal comparator plans, memoized by `root_frame_depth` and
       (pattern span, request ordinal). Any other descent write is a stop
       to the Architect.
     - Propagation: census every handler from `compile_match_leaf` to the
       owning entry. A catch-all on that path, or a leaf that adds an `env`
       declaration, is a stop to the Architect.
   - In pure inference mode, an annotation is required only when the solved
     R would mention a derived-telescope binder. The precise diagnostic is
     raised there; "the split reverts" is not the test.
   - Occurrence terms replace sentinel depth arithmetic, and the four
     mechanisms above are deleted.
   - The in-matrix alias finalize and per-method kernel check run in the
     derived telescope on every path. Both `needs_reverting` gates are
     removed (`evt_4c4tgkc2gvypk`, `evt_qh7m7erbc5f6`, input f).
   - **Root-frame ownership** (increment I-2a, merged `d7676f128`, exact
     `7effbe9a4`; Architect carry `evt_5ezxyycetagrw`, approval
     `evt_7904vfyy22b57`). Today
     `memoize_indexed_root_motive` writes `indexed_match_roots.last()`, so an
     inner match in an indexed root's first leaf can write the outer root's
     motive. Both writers, the entry seed and the first-leaf producer, take
     `root_frame_depth`, and write only when `indexed_match_roots.len() ==
     depth + 1`. Fixtures p1 (inner inference match) and p2 (inner checked
     match) give `Suc Zero` from `f Zero (VNil Nat)`. Both fail closed on
     main with `KernelRejected TypeMismatch`.
   - **Alias occurrences and the checked path** (Architect
     `evt_635vc9mxvmnq2`, on the checked-sibling stop). The sentinel's only
     producer is `materialize_pattern_alias`'s sentinel arm; the flat
     checked path only consumes it.
     - INV-1: an alias use materializes as an occurrence term, in the same
       representation as a plain reference to its binders.
     - INV-2: the checked path gets no design change. Its edits are the
       mechanical fallout of deleting the sentinel type, which is in scope:
       the `wrap_premise_*_finalized` pair returns `Term` again, and the ten
       `AliasAcrossPremiseWrap` `map_err` arms go. The index-refinement
       sentinels stay byte-identical. Any other edit in the checked or
       convoy builders is a stop to the Architect.
     - Migration: every sentinel constant, finalizer, guard and
       `PatternVariableAcrossDependentSplit` goes in the same commit that
       deletes the producer arm, never earlier.
   - Representation (input g, amended Architect `evt_216r68j77vfdx`):
     single coordinate system. Every binder the matrix emits is a `cx.ctx`
     push, hidden when it carries no surface name. Each row occurrence is
     shifted by ordinary `weaken` exactly once per push. Surface names
     resolve by position identity (`surface_binding_target`).
     `check_nested_index_variables` refuses any reorder of bound entries.
     There are no elaborator-only free variables and no `abstract`. The
     kernel `Term` is unchanged.
     - C1: `weaken_woven`, `under_woven_binder` and
       `enter_woven_real_binder` are deleted in the same commit, and the
       comments they falsify are corrected.
     - C2: mutations M-A (the `Ih` arm keeps its push, drops
       `under_core_binder`) and M-B (the split arm drops its push) each
       redden a named test; a wrong value reddens a discriminating value pin.
       A green mutation is a stop to the Architect.
     - C3: census every nested-matrix site that abstracts or re-binds an
       entry already in `cx.ctx`. Expected none; any one is a stop to the
       Architect.

## Acceptance

- **AC-0a (increment 0; merged `457898bcb`, exact `2bdafdc29`).**
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
  - **Check path.**
    - The exact checked R2 (`xs : Vec Nat n`, the Adversary's shape) never
      reaches a non-empty wrap. Its pin is `ElabError::KernelRejected`
      carrying `VarOutOfScope { index, .. }` with `index ==
      PATTERN_ALIAS_SENTINEL_BASE`, asserted through the constant, at the
      VNil span (Architect `evt_3cqccrymnd3n0`). The test comment says the
      kernel is refusing an unresolved sentinel, and that a typed diagnostic
      is an increment-2 residual. Do not retype the elaborator's
      `KernelRejected` sites.
    - The checked single-arm sibling (`Vec Nat (Suc n)`) refuses with
      `PatternVariableAcrossDependentSplit` at the wrap. Its test comment
      keeps the same-typed census (saved=3, j=2, e=5, {n,m}=0).
    - Base census on `origin/main`:
      - the sibling: `Zero` is the live base miscompile this increment
        turns into a refusal. If main refuses or gives 3, stop to the
        Architect.
      - the exact R2: if main gives a value, report it and the binder it
        selected. If main gives the same raw rejection, record "unchanged".
    - The fan-in table also names the elaborator function, as file:line,
      that issues the VNil-span kernel check. Nothing is built there.
    - Unit rows call both wraps directly. A sentinel with one premise gives
      `Err`. The same sentinel with zero premises gives `Ok`. A
      sentinel-free body with premises gives `Ok`, byte-identical to the
      output before the change.
    - c1 and the seven controls keep their values. A control that now
      refuses is a stop to the Architect.
    - Mutation: an always-`Ok` guard reddens the unit rows and the checked
      sibling. The exact R2 pin stays green, since it refuses before any
      wrap. The infer rows still refuse through the finisher guards.
    - The fan-in table lists all ten wrap call sites with their span source,
      plus the `from_full` exclusion.
- **AC-1b (increment 1b).**
  - The three Adversary rows (tuple, or-pattern, nested `Nat`), elaborated
    and normalized, are Ok. Each is `KernelRejected` on `2d6b64aca`.
  - A record-path row with the same `: Type` result. Probe it first. If it
    is refused at both `a246ede23` and `2d6b64aca`, report it and do not pin
    it: it is LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE's.
  - The increment-1 first-bucket unit pin still observes `[true]`. Re-run
    the increment-1 census: a check-mode entry that was `Some` and turns
    `None` is a stop to the Architect.
  - Mutation: an always-seed helper returns all three rows to
    `KernelRejected`.
  - Controls stay Ok: an explicit `: Type 1`, `Type 0` arms, inference
    mode. The plain `match b { True ↦ Type; False ↦ Type }` stays refused
    at both SHAs.
- **AC-1.** The M-deep Zero fixture and its two-field sibling flip from
  transition sentinel to their normalized values.
  Increment 0's refusal pins flip from refusal to 3, built from occurrence
  terms in the derived telescope (Architect `evt_1f0xnnv11654x`): all 11
  adversary rows including R1 and R2, the exact checked R2, the checked
  single-arm sibling, and the Eq-index `Ix/Mk` row (the infer-finisher
  discriminator, `evt_57fd0ncxh81v8`). The exact R2's raw sentinel-base
  `VarOutOfScope` gives way to a value; if it still refuses, its diagnostic
  is typed, not a raw kernel rejection.
- **AC-2 (controls).**
  - Every value pin of `LANG-NESTED-SPLIT-FIELD-DEPENDENCE` stays green.
    That includes the collision control and M1, M2 and M3.
  - A dependent-R fixture in inference mode gives the precise diagnostic.
  - A constant-R fixture in inference mode still checks.
  - The `lang_match_record_pattern` record row and the reverting
    woven-column fixture (FIELD-DEPENDENCE P5), the Dep row and the deferred
    wrong-method fixture are discriminating value pins, checked in-matrix.
  - `PatternVariableAcrossDependentSplit` is removed (input h).
  - The checked single-arm sibling (`let r : Nat = match xs { VCons m e tl
    ↦ ... }`) stays green with arm bodies `Suc j` ⇒ 3, `j` ⇒ 2 and `e` ⇒ 5,
    and the nested-woven `BoxNat (Suc saved)` row ⇒ 3; AC-1's `saved` row ⇒
    3 is the flip (`evt_635vc9mxvmnq2`).
- **AC-2b (inference-mode R, `evt_9s1tts2m0xjk`).**
  - `lang_nested_split_field_dependence.rs:36-52` (VCons first) checks with
    both values. Leaving the slot `None` at the `Ih` column reddens it.
  - R identity census: for every inference-mode entry in the named suites,
    report the rerun count. Discovered R is byte-identical to the R that
    `d7676f128`'s first-leaf code computes.
  - Report the maximum nested-rerun depth across the suites and the catalog.
- **AC-3.** `lang_infer_match_indexed_complete` and the as-pattern,
  nested-split and tuple-pattern suites stay green. The catalog census is
  byte-identical, and `trusted_base()` is unchanged.

## Symptom inventory (§1b, Architect)

1. An enclosing alias sentinel reaches an elaborator-to-kernel call
   unresolved, before its frame finishes. Keyed on kernel-call reachability
   of an unresolved sentinel (`evt_4416jtap9bj62`, increment 0 §1a 1;
   Architect `evt_3cqccrymnd3n0`).
2. Inference-mode R is needed before the first leaf under an IH column; the
   frame named a term metavariable the elaborator lacks. Keyed on an enabler
   absent from the delivered vocabulary (`MetaCtx` is level-only;
   `evt_44yy0cg0qb448`, §1a 1, `evt_9s1tts2m0xjk`). The §1b test is due at
   entry 3.

Check-mode result seed (a separate count, §1a 1, `evt_6jqa2y7rft3tr`):

1. The check-mode seed read an unsolved level meta as Zero. Keyed on the
   goal's levels being solved.

## Stop conditions

- The repair needs a kernel or spec change.
- Seeding leaves an entry `None` in check mode: stop and name it.
- A retired mechanism has a consumer outside the matrix: stop and name it.
