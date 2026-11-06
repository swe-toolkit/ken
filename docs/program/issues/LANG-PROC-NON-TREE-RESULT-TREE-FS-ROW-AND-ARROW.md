---
id: LANG-PROC-NON-TREE-RESULT-TREE-FS-ROW-AND-ARROW
title: "LANG-PROC-NON-TREE-RESULT-TREE gives a non-tree-result proc its §2.5 tree only for closed ambient rows (Console, Clock, Entropy) and non-function results. Procs with an FS label, an open row, or a function-valued declared result keep plain B, and a performing let at their outer level is refused as an elaboration gap. Give those procs the tree the spec rules and delete the interim refusal"
status: active
owner: verify
size: L
tier: T1
gate: architect
depends_on: [LANG-PROC-NON-TREE-RESULT-TREE, SPEC-EFFECTS-DECLARED-TREE-RESULT-DENOTATION]
blocks: []
github: null
origin: "Architect AC-0 fork on LANG-PROC-NON-TREE-RESULT-TREE (evt_2f6y8k5n184e4), Steward scope decision (b) evt_4c6nvqtavtgg8, Architect re-rule evt_46t5492bw0fsm adding arrow-valued results to this node. Steward-filed per COORDINATION section 2."
---

# Every performing proc returns its tree

## Objective

Every non-space-op `visits` proc with a non-empty row and a non-tree
declared result denotes the tree spec 36 §2.5 rules for it, including
procs with an `FS` label, an open row, or a function-valued declared
result. `refuse_unsequenced_performed_let` is deleted.

## Settled inputs (to be re-anchored on the landed base at kickoff)

- **The first slice.** LANG-PROC-NON-TREE-RESULT-TREE Inc 1 adds
  `returned_tree_signature`, `check_returned_tail` and the closed
  inclusion table for the ambient rows (Architect `evt_2f6y8k5n184e4`,
  item 1 re-ruled `evt_46t5492bw0fsm`). It narrows the interim refusal
  to the rows this node covers.
- **The measured population** (the LANG-PROC sweep, 45/45 binaries):
  - 12 [FS] procs: `ok_use`, `step_int`, `compute_effectful`,
    `effectful_seed`, `ok`, `Client.ok`, `step_bool`, `A.f`,
    `Good.good`, `surf1_read`, `surf1_load_ok` and `fs_source`;
  - 3 arrow-valued declarations in 2 files: `selected_body` (px7l and
    rt_selected_pending_call_admission) and `selected_result` (px7l);
  - row variables: static only, `r_layer_tests::effects` and
    `match_arm_glyph`.
- **The open work** named by the Architect: the [FS] tree's Auth index;
  §2.4's argument sequencing (`g a1 … an`) for an arrow-valued result
  used in an argument slot; and a native plan for a `Ret` that carries
  a closure.
- **Transition sentinels** (Architect `evt_3dqgtgqp18tbn`). A
  [Console]-row proc that sequences a callback whose declared tree is
  `HostIO` refuses with E0087, because no inclusion maps `HostIO` into
  `IO`. The pins are `r11` (lang_effect_row_match_binder_latent) and
  `p3`, `r1` and `r1k` (lang_effect_row_cell_scrutinee_carried_row).
  If the spec node rules that a [Console]-latent callback's tree is
  `IO`, they flip to acceptance here.
- **The `ensures` sentinel** (Architect `evt_1vmhvzqhrv6ak`). Inc 1
  refuses `ensures` on a changed proc with E0038 and the text "`ensures`
  cannot be checked here", pinned by
  `ensured_changed_proc_refuses_until_tree_post_is_specified`. It flips to
  acceptance here, under the Spec node's relational post.
- **E0087's text** prints raw core terms, so the user sees ids rather
  than `HostIO` and `IO` (`evt_3dqgtgqp18tbn` item 5).
- **Scrutinee and argument slots** (Architect `evt_688spjn83adpq`).
  Inc 1 sequences a tree only at a let and a returned tail, so a
  tree-typed recursive result in a scrutinee refuses E0038 (`r1`, its
  sentinel), and that diagnostic names `ITree` where the user wrote
  `Box`.
- **A carried doc fix** (Architect `evt_11k493kejypt9`, carry 1). At
  `prelude.rs:3572` on Inc 1, `capture_effect_tree_ids` sits between
  `register_prelude`'s doc comment and `pub fn register_prelude`, so the
  comment documents the private helper. If this node touches
  `prelude.rs`, move the helper above the comment and give it its own
  one-line doc.
- **The spec** is SPEC-EFFECTS-DECLARED-TREE-RESULT-DENOTATION. This
  node is not released until that node lands.

## Deliverable

The ruled tree for each covered row and result shape, the caller splice
it needs, the interim refusal deleted, and the consumer migration, in
the increments AC-0 rules.

## Acceptance

- **AC-0.** The Architect specifies a sweep D0 on the landed base and
  the Verify implementer runs it on scratch, as in the parent WP's AC-0.
  The Architect rules the elaboration per covered case, the argument
  slot, any runtime pin that moves, and the increment split. A runtime
  plan change is a stop to the Steward for placement.
- **AC-1.** A performing let at a proc's outer level sequences in each
  of these shapes, as spec 36 §2.5 rules them (Architect
  `evt_6qwshneax61z8`):
  - `[FS]` with a written `c : Cap a`, an FS action: `FS a B`;
  - `[FS]` plus an ambient label, with the Cap: `HostIO a B`;
  - `[FS, Console]` without a Cap, a Console action: `IO B`;
  - an arrow-valued proc whose final result is a non-tree: inside the
    last lambda, against the wrapped final result.
- **AC-1r.** Each of these refuses with a §2.5 message, not the interim
  gap: the parent's R4 (an outer action before the last arrow); an FS
  action in a free proc with no written Cap; two written Caps with
  different Auth indices; and a declared-tree proc with a label that
  has no summand.
- **AC-2.** A mutation that restores the narrowed refusal reddens AC-1.

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A change to a declared-tree program's core type.
