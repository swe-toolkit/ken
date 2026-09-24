---
id: KERNEL-LITERAL-ROLLBACK-PURGE
title: "Soundness repair for K3: rolling back a declaration must drop its checked String literal payload, so a later declaration that reuses the freed id cannot inherit it -- the kernel must not convert string_to_list_char of an unrelated foreign String to a stale literal's characters, and the interpreter must not evaluate an Int definition to a String"
status: merged
owner: kernel
size: S
gate: architect
tier: T1
depends_on: [KERNEL-LITERAL-CHAR-VIEW]
blocks: []
github: null
origin: "Adversary M8 hunt on K3 squash bfdbb9789 (evt_3e1de9jrqwbmq): SOUNDNESS. Repair of the operator-authorized K3 (2026-09-24, 'concur with rec.'); it adds no trust. Steward-filed per COORDINATION section 2."
---

# A rolled-back literal outlives its declaration

## Fixed inputs -- measured at `bfdbb9789`

- `GlobalEnv::remove_last` (`crates/ken-kernel/src/env.rs:465`) pops the
  last declaration, rewinds `next_id`, and purges `by_id`, `ctor_index`,
  `terminal_supports`, `support_edges` and `all_supports`. It does **not**
  purge `checked_literals` (the K3 field, `env.rs:313`).
- The elaborator's rollback loops (`while let Some(d) = env.remove_last()`,
  for example in `elaborate_recursive_view`) pop literal declarations that a
  failed body added. The next declaration to receive a freed id inherits the
  stale payload.
- The K3 whnf arm in `conv.rs` and `eval.rs`'s `Const` case key on
  `checked_literal(id)` alone. Neither checks that the id is still a live
  `PrimReduction::Literal` declaration at the checked String carrier.
- The Adversary's measurements are unmeasured by the ring; re-establish them:
  - At `bfdbb9789`, a false
    `Equal (List Char) (string_to_list_char p1) (string_to_list_char zz_lit)
    = Refl` over a `foreign p1 : String` that landed on the freed id is
    **accepted**. The same sequence at the parent `88124a613` rejects it.
  - At runtime, `const i1 : Int = 7` on the freed id evaluates to
    `Str("zz")`.
- `elab.rs::elab_str_lit` inserts the same checked String id into both the
  kernel `checked_literals` table and the elaborator `num_values`
  (`NumericLitVal::Str`). `eval.rs::eval` checks the kernel payload first,
  then `num_values`. `compiler_driver.rs::literal_native_symbol` has the same
  fallback. The `num_values` half may predate K3 and is unmeasured at the
  parent. Char literals use core `IntLit` and have no table entry; no Char
  repair is needed.
- `GlobalEnv::checked_literal` is public and does a raw table lookup. Its
  consumers are `conv.rs`, `ken-interp/src/eval.rs` and
  `ken-elaborator/src/compiler_driver.rs`, plus fresh-registration code.
- Frame boundary: Architect `evt_5b8cekxazqhtd`. Both tables share one
  rollback-lifetime invariant, and it is repaired here as one node.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

One rollback-lifetime invariant across both literal tables:

1. `remove_last` purges the popped id's `checked_literals` entry, beside the
   tables it already purges.
2. `GlobalEnv::checked_literal` returns a payload only when the id is a live
   `Decl::Primitive { reduction: Literal, ty }` whose `ty` is exactly the
   registered String carrier. A type that merely converts to String is not
   the carrier. If the guard lives in the consumers instead, it must cover
   all three, including `compiler_driver.rs`.
3. The elaborator rollback sites drop `num_values` entries for the ids they
   pop.

No new reduction, registration path, primitive, postulate or trust entry.
The K3 acceptance tests stay green without changed expectations.

## Acceptance

- **AC-1 (parent/base matrix, red first).** Commit the Adversary's sequence
  (`ElabEnv::elaborate_decl` in order) as a regression. Record:
  - at parent `88124a613` and base `bfdbb9789`: the false `Refl` over the
    reused-id foreign String (expected rejected at parent, accepted at base);
  - at parent and base, using only facilities the parent has: the runtime
    stale side table, where the reused-id `Int` const evaluates to
    `Str("zz")` (possibly older than K3; measure it, do not impute it);
  - at base only: the positive, a genuine literal declared after the
    rollback gets its own checked `string_to_list_char` view. The parent
    predates K3 and has no such view.
  Every negative is paired with a reaching positive; no bare `expect_err`.
- **AC-2 (candidate).** The false theorem rejects at its `Refl` obligation,
  and so do the adjacent controls. Each popped-id reuse evaluates to its own
  value. A fresh legitimate String and a fresh non-String literal after
  rollback both behave correctly. The native metadata observer agrees with
  the checked literal provenance.
- **AC-3 (single-arm mutation controls; count the executed tests).**
  - **Purge.** A kernel-local test asserts on the *raw* table, right after
    `remove_last` and before id reuse, that the popped id is absent. The
    guarded accessor must not stand in for this assertion. Deleting only the
    purge reddens it, even though the guard masks the stale entry
    downstream.
  - **Live-declaration guard.** Keep the purge. Seed a stale payload
    directly on a live declaration of the wrong kind or the wrong String
    carrier, using kernel-internal test machinery. Deleting only the guard
    reddens this control.
  - **`num_values` cleanup.** Keep the purge and the guard. Deleting only
    the rollback-site cleanup reddens the runtime reused-`Int` test.
- **AC-4.** No new trust. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

Stop if the base does not reproduce the kernel row, if the repair needs a
new kernel reducer or a change to literal registration, or if any K3 AC test
has to change its expectation.
