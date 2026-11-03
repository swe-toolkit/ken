---
id: LANG-PROC-NON-TREE-RESULT-TREE
title: "Spec 36 §2.5 elaborates `proc f (x : A) : B visits ρ` to `f↓ : caps → ⟦A⟧ → ITree ⟦ρ⟧ ⟦B⟧`, but a proc whose declared result is a non-tree type such as Int elaborates to plain B, so its body has no tree to sequence a performing let into. LANG-PROC-NON-TREE-LET-SEQUENCING refuses that let at check time as an elaboration gap. Make a non-empty-row proc's result the §2.5 tree, with callers splicing it by §2.4's incl, and lift the interim refusal"
status: ready
owner: verify
size: L
tier: T1
gate: architect
depends_on: [LANG-PROC-NON-TREE-LET-SEQUENCING, LANG-EFFECT-ROW-MATCH-BINDER-LATENT, LANG-EFFECT-ROW-LET-BOUND-SCRUTINEE-LATENT, LANG-EFFECT-ROW-CELL-SCRUTINEE-CARRIED-ROW]
blocks: []
github: null
origin: "Architect stop evt_axd6p7yc8d88 on LANG-PROC-NON-TREE-LET-SEQUENCING, option A; Steward ruling evt_j0vvt2xr28c1 filed it as its own node, sequenced after LANG-EFFECT-ROW-MATCH-BINDER-LATENT on the Verify ring. The interim refusal landed in da6146038. Steward-filed per COORDINATION section 2."
---

# A performing proc returns its tree

## Objective

A `visits` proc with a non-empty row and a non-tree declared result
denotes `ITree ⟦ρ⟧ ⟦B⟧` as §2.5 states, and its callers splice it in by
§2.4's call rule. A performing let in its body then runs: the
NON-TREE-LET probe prints `ab` and a newline, and the interim refusal is
deleted.

## Settled inputs (main `f53d7cfb5`)

- **The spec.** §2.5 (`36-effects.md:649` to `:652`) gives the tree
  result. §2.4 (`:600` to `:620`) gives `⟦ g a1 … an ⟧` as a bind
  ending in `incl_{ρ_g ↪ ρ} (g↓ caps x1 … xn)`, a pure `elim_ITree` map.
- **As built** (Architect D0 on `fcc816c65`, evt_axd6p7yc8d88). Given
  `proc read : Int visits [Console] = 1`:
  - `proc caller : Int visits [Console] = read` checks, so `read : Int`;
  - `proc caller2 : IO Int visits [Console] = read` is refused with
    TypeMismatch, so `read` is not a tree;
  - a body that performs directly against `Int` is refused.
- **The interim refusal** this WP deletes:
  `refuse_unsequenced_performed_let` (`elab.rs:2259`), called from
  `refine_checked_let_rhs` (`:2288`) and `prepare_inferred_let_rhs`
  (`:14474`). Its pins are R1 to R3 in
  `crates/ken-cli/tests/lang_proc_non_tree_let.rs`.
- **The population, a lower bound.** A single-line grep of
  non-tree-result `visits` proc signatures finds about 40 in 14 files.
  The largest is `surf_space_cells_p1.rs` with 14. Others are
  `seed-class-field-purity.md`, `seed-effects.md`,
  `lang_let_hostio_action.rs`, `class_field_purity.rs`,
  `ds8b_pure_into_proc_widening.rs` and `purity_keywords.rs`.
  Multi-line signatures are missed, and every caller of each also
  changes.
- **The row-M frame budget.** The let fallbacks sit on a recursive frame
  that `elab_recursive_frame_budget` row M measures at 3,072 KiB, depth
  48 (Architect evt_1m6eavgznkmq9).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The §2.5 result and the §2.4 caller splice for non-empty-row procs, the
interim refusal deleted, and the consumer migration, in the increments
AC-0 rules.

## Acceptance

- **AC-0.** The Spec Leader confirms §2.5 as the normative target, and
  rules what a non-tree-result `visits` proc's declared type means at
  the surface. The Architect then rules:
  - the elaboration site;
  - the caller splice and its fan-in: direct calls, partial
    application, use as a value and method fields;
  - `proc direct_value : Int visits [Console] = 1`, which becomes
    `Ret 1` in core;
  - the consumer sweep, with a disposition per file. The Architect
    specifies the run and the Verify implementer executes it on
    scratch, so the Architect rules from the reported result and does
    not build or test.
  - the frame-budget effect;
  - the increment split.
- **AC-1.** The NON-TREE-LET probe prints `ab` and a newline. R1 to R3
  flip from refusal to the ruled acceptance, and a caller of `read`
  still gets `Int` at the surface where AC-0 rules it does.
- **AC-2.** A mutation that restores the plain-B result makes AC-1 red.
- **AC-3.** Every sweep consumer checks and runs as before, or changes
  as AC-0 names. `trusted_base()` is unchanged.

## Stop conditions

- A ruling that the spec should adopt direct style instead of §2.5.
  That is a language-design change and returns to the Steward.
- A kernel or trust change, or a raised stack constant.
- A consumer whose output changes in a way AC-0 did not name.

Checks: 4 fired: the obligation is written in the code's own types. The
bind is `ITree Console Int` against a plain `Int` result, which needs
the result to be a tree, and §2.4's `incl` is the splice that already
exists. 3 fired: the population is a grep over catalog, conformance,
examples, crates tests and r_layer_tests, stated as a single-line lower
bound, with the run-based sweep left to AC-0.
