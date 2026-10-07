---
id: KERNEL-ENV-MARK-PREFIX-MUTATION-REFUSAL
title: "rollback_to_mark and rollback_pending succeed after a prefix declaration was upgraded to a body that references the removed suffix, leaving a Transparent declaration whose body dangles and then aliases a reused GlobalId of another type. Refuse rollback when the prefix changed after the mark, so the EnvMark contract holds"
status: merged
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [VERIFY-REUSED-ENV-TRUST-RESIDUE]
blocks: []
github: null
origin: "Adversary finding evt_4b2mzf1x5erza on 174904b18. The repair keeps the operator-approved removal-only EnvMark API (2026-10-05) to its stated contract; it adds a refusal and no trust. Steward-filed per COORDINATION section 2."
---

# An EnvMark rollback leaves a dangling upgraded body

## Objective

`rollback_to_mark` and `rollback_pending` never leave a declaration whose
body references a removed id. A rollback that would do so is refused.

## Settled inputs (Adversary `evt_4b2mzf1x5erza`, on `174904b18`)

- **Repro, public `ken_kernel` calls only.**
  1. `p = declare_postulate(env, "p", [], Top)`.
  2. `mark = env_mark(&env)`.
  3. `d = declare_def(env, [], Top, tt)`, at g4.
  4. `admit_bodies(env, &[(p, Const d)])` is Ok.
  5. `rollback_to_mark(env, mark)` is Ok and removes g4. `p` stays
     `Transparent { ty: Top, body: g4 }`.
  6. `e = declare_def(env, [], Ω0, Top)` reuses g4.

  Afterwards `p`'s body fails its own type (`TypeMismatch { expected: Top,
  found: Ω0 }`), `check(tt : p)` is Ok, and `p` has left `trusted_base()`.
- **The guard.** `environment_mark_prefix_valid` (`check.rs:1441`) checks
  only that prefix ids are below `mark_next_id`. The prefix mutation is
  `upgrade_to_transparent` (`env.rs:1007`) through `admit_bodies`
  (`check.rs:1634`). The same sequence through `stage_placeholders` and
  `rollback_pending` reproduces.
- **Reachability.** No elaborator or CLI path calls `admit_bodies`. Every
  elaborator staging pair sits inside its mark (`lib.rs:646`,
  `modules.rs:3102`, `4191`, `4359`).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Repair hypothesis: the mark records enough to detect that a prefix
declaration was upgraded after it, and both rollbacks refuse with the
existing prefix-no-longer-holds error before removing anything. The
Architect may rule a different shape, such as restoring the assumption,
before implementation.

The sibling the Adversary names, an index registration of a prefix
declaration made after the mark (`register_unit_type`, the int-literal
carrier, the literal char view), breaks index restoration but not typing.
Measure whether any elaborator rollback reaches it, and report it to the
Architect. Repair it only if ruled.

## Acceptance

- **AC-1.** The repro above is a kernel test: step 5 returns the existing
  prefix error, and the environment is unchanged from immediately before
  step 5. `p` stays `Transparent { body: Const d }`, `d` stays present,
  `p`'s body checks against its type, and `trusted_base()` is equal across
  the refusal (`p` is absent from it since step 4). The `rollback_pending`
  variant is refused the same way.
- **AC-2 (controls).** The existing `env_mark` and rollback tests, and the
  elaborator rollback suites from `VERIFY-REUSED-ENV-TRUST-RESIDUE`, keep
  their results.
- **AC-3 (mutation, QA).** Removing the new check makes AC-1 red at
  step 5, for both rollbacks.

## Stop conditions

- Any trust growth, spec change or new kernel entry point beyond the
  refusal.
- An elaborator path that the refusal would newly break.

## Closeout

Merged `26e7dce3f` from exact `1b0b55acf` (PR #4568). Kernel QA
`evt_f9t54y7c25z3`, Architect `evt_5qpyaa2jw7dmq`, Decision
`dec_3yfarp0frhr9d`. The mark records the prefix's opaque ids, and both
rollbacks refuse before removing anything when one was upgraded. The
refusal also covers an upgrade whose body references only the prefix, and
no production path hits it. Residuals: a post-mark index registration on
a prefix id is cleared rather than restored on rollback; it fails closed,
and no elaborator rollback reaches it. `env_mark` now costs O(decls).
