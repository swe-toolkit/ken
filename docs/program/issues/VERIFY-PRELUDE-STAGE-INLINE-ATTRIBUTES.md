---
id: VERIFY-PRELUDE-STAGE-INLINE-ATTRIBUTES
title: "LANG-ELAB-RECURSIVE-FRAME-BUDGET left three prelude stages with a duplicated #[inline(never)] and three siblings with none. rustc is moving the duplicate toward a hard error. Place one attribute per stage and remeasure the prelude frame"
status: active
owner: verify
size: S
tier: T2
gate: architect
depends_on: [LANG-ELAB-RECURSIVE-FRAME-BUDGET]
blocks: []
github: null
origin: "Architect evt_3ea7andmw55wy, approving LANG-ELAB-RECURSIVE-FRAME-BUDGET 3b776f307: the duplicated attributes are a non-blocking residual carried to the Steward as a named follow-up; a toolchain bump would fail to compile them. Verify QA evt_31xp6tk59p9af reported the warnings. Steward-filed per COORDINATION section 2."
---

# One inline attribute per prelude stage

## Objective

Every `register_prelude_*` stage in `crates/ken-elaborator/src/prelude.rs`
carries exactly one `#[inline(never)]`. The elaborator builds with no
`unused_attributes` or `unnecessary_braces` warning from this code, and the
prelude still loads within the stack that LANG-ELAB-RECURSIVE-FRAME-BUDGET
set.

## Settled inputs (on `3b776f307`)

- **Duplicated:** `register_prelude_pairs` (L1151-1152),
  `register_prelude_resource_seed` (L1934-1935) and
  `register_prelude_fs_operations` (L2461-2462) each carry two
  `#[inline(never)]` lines.
- **Missing:** `register_prelude_propositions` (L1248),
  `register_prelude_resource_shapes` (L2425) and
  `register_prelude_resource_protocol` (L3080) carry none. The Architect
  read the pattern as misplaced copies.
- **Braces:** `modules.rs:2707` wraps `rewrite_rexpr_arm(..)?` in
  `Ok({ .. })`.
- **The frame claim:** `register_prelude` is 6,408 B, its largest stage is
  32,632 B (under 32,768), and the prelude and 71 catalog packages load at
  1,664 KiB. Moving an attribute changes the frame layout, so these must be
  remeasured, not assumed.

Treat line anchors as perishable. Re-read them on the landed base.

## Deliverable

The attribute placement and the braces fix in `prelude.rs` and
`modules.rs`, with the guard file's stated numbers updated only if the
remeasure moves them.

## Acceptance

- **AC-1.** A targeted `scripts/ken-cargo build -p ken-elaborator` shows no
  `unused_attributes` or `unnecessary_braces` warning from these sites.
- **AC-2.** The prelude, and every catalog package, load at 1,664 KiB.
  Every `register_prelude_*` stage stays under 32,768 B, and the largest is
  named in the handoff.
- **AC-3.** `elab_recursive_frame_budget` passes 2/2, unchanged.

## Stop conditions

- A newly attributed stage pushes `register_prelude` or a stage past its
  ceiling, or the prelude no longer loads at 1,664 KiB.
- The fix needs a change outside these two source files, other than
  re-stating measured numbers in the guard file.
