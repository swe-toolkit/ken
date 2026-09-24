---
id: KERNEL-LITERAL-ROLLBACK-PURGE
title: "Soundness repair for K3: rolling back a declaration must drop its checked String literal payload, so a later declaration that reuses the freed id cannot inherit it -- the kernel must not convert string_to_list_char of an unrelated foreign String to a stale literal's characters, and the interpreter must not evaluate an Int definition to a String"
status: ready
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
- The elaborator's `num_values` entries for popped literal ids also survive
  rollback. This half likely predates K3 and is unmeasured at the parent.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. `remove_last` purges the popped id's `checked_literals` entry, beside the
   tables it already purges.
2. The kernel conversion arm and the interpreter's literal lookup accept a
   payload only when the id is a live `PrimReduction::Literal` declaration at
   the checked carrier. The table alone is never authoritative.
3. The elaborator rollback sites drop `num_values` entries for the ids they
   pop.

No new reduction, primitive, postulate or trust entry. The K3 acceptance
tests stay green.

## Acceptance

- **AC-1 (base red first).** Commit the Adversary's sequence as a
  regression test (`ElabEnv::elaborate_decl` in order): it is red at
  `bfdbb9789`, with the false `Refl` accepted only for the reused id and
  rejected for the adjacent controls. Include the runtime `Int` and `String`
  cases.
- **AC-2 (candidate).**
  - Kernel side: the false theorem rejects, and so does every adjacent
    control.
  - Runtime side: each popped-id reuse evaluates to its own value.
  - A genuine literal declared after the rollback still gets its own view.
- **AC-3 (mutation controls).** Each is run alone:
  - Remove only the `remove_last` purge: the kernel case reddens, or else it
    stays green because deliverable 2 blocks it; the handback says which.
  - Remove only the live-declaration check: the direct stale-table test
    reddens.
- **AC-4.** No new trust. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

Stop if the base does not reproduce the acceptance, if the repair needs a
new kernel reducer or a change to literal registration, or if any K3 AC test
has to change its expectation.
