---
id: KERNEL-ADMIT-BODIES-CHECKED-UPGRADE
title: "A body can leave trusted_base() without SCT or a cycle check: discharge_hole accepts the hole itself as its certificate, because the raw pub upgrade_to_transparent is gated only by its callers. Move the gate into the kernel as one checked admit_bodies and make the raw upgrade crate-private"
status: merged
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE]
blocks: []
github: null
origin: "Adversary finding evt_2dcmesxd8395a (M8 hunt on 768448f25), classified by the Architect in evt_1wkd0t4w3e2zj as a latent kernel trust-soundness defect needing its own node. Steward-filed per COORDINATION section 2. It narrows the kernel's public API and relocates a trust gate into the TCB, so it waits on an operator decision."
---

# One checked admission gate for transparent bodies

## Objective

No body can move a declaration out of `trusted_base()` unless the kernel
itself has checked the body, run SCT on it, and ruled out a cycle through
the group. Today the report is only as honest as every caller that
remembers the gate.

## Fixed inputs (Architect `evt_1wkd0t4w3e2zj`, read at `768448f25`)

- **The defect.** `ElabEnv::discharge_hole`
  (`crates/ken-elaborator/src/lib.rs:552-558`) runs
  `kernel_check(cert : goal)`, then calls
  `GlobalEnv::upgrade_to_transparent` (`crates/ken-kernel/src/env.rs`)
  directly.
  - `Const(hole)` has type `goal`, so `hole := hole` is admitted.
  - With `goal = False`, the kernel reports a closed inhabitant of False
    with no assumption. That breaks 21 §6.5.
- **The indirect variant.** A certificate that mentions an `f` whose body
  references the hole creates `hole → f → hole`. SCT run on the singleton
  group accepts it.
- **The fan-in.** These callers gate the upgrade themselves:
  - `declare_def` and `declare_recursive_group` in `check.rs`;
  - elaborator sites `elab.rs:13955`, `:14214` and `:14550`.

  These callers bypass it: `discharge_hole`, and `b1_acceptance.rs:169`,
  which calls the raw upgrade.
- **The plane.** CHECK-REFL increment 2's `body_refs` index in `env.rs`
  carries the forward references the escape check reads.
- **Reachability.** Only tests call `discharge_hole` today. V2 and the
  protocol (spec 22 and 25) are its intended callers.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch.

## Deliverable

The design is the Architect's ruling:
- `upgrade_to_transparent` becomes `pub(crate)`.
- `check.rs` gains
  `pub fn admit_bodies(env: &mut GlobalEnv, group: &[(GlobalId, Term)])
  -> KernelResult<()>`. It runs these steps in order:
  1. Check each member is a present `Decl::Opaque` and its body checks
     against its type.
  2. Run `sct_check` on the group.
  3. The escape check: walk forward over transparent bodies from each body,
     not continuing through members. Refuse if the walk reaches a member
     after at least one non-member step.
  4. Only then upgrade all members.
- `declare_def`, `declare_recursive_group`, the three elaborator sites and
  `discharge_hole` call `admit_bodies`. `b1_acceptance.rs:169` migrates to
  it.

## Acceptance

- **AC-0 (caller census; no build).** List every use of
  `upgrade_to_transparent` and every test that calls it, at the landed
  main. Any caller the list above misses is a stop to the Architect.
- **AC-1.**
  - `hole := hole` is refused, and the hole stays in `trusted_base()`.
  - The indirect `hole → f → hole` is refused, and the hole stays.
  - An honest certificate discharges, and the hole leaves.
  - Self-recursive and mutual definitions still admit.
  - A compile check shows no crate outside `ken-kernel` can name
    `upgrade_to_transparent`.
- **AC-2 (controls).**
  - Removing the escape check admits the indirect case.
  - Removing SCT admits the direct case.
  - The 57-package census shows no verdict change.

## Stop conditions

- Any change to what the kernel accepts, other than refusing a circular
  certificate: stop to the Architect.
- Any spec change: an operator question.
- A `trusted_base()` change beyond keeping a circular-discharged hole in it.

## Closeout

Merged as `84dc2f602` (PR #4384).
- **AC-0.** The census found 15 call sites, not the 7 named. That stopped
  to the Architect, whose amended ruling disposed of all 15.
- **AC-1.** `check::admit_bodies` is the one gate, and
  `upgrade_to_transparent` is `pub(crate)`.
  - `admit_bodies.rs` (7 tests): `hole := hole` is refused, and so are the
    indirect bridge and the cross-member escape. A late invalid member
    leaves the group opaque. An honest discharge retires only its own id.
  - In `v1_acceptance.rs`, both circular certificates leave the hole in
    `trusted_base()` with the env unchanged.
- **AC-2.** The Kernel QA mutations each redden their row:
  - omitting SCT reddens the direct case;
  - truncating reachability reddens the indirect and cross-member cases;
  - omitting the body check reddens the ill-typed cases.

  The 57-package census shows no verdict change.
- **Carry (later kernel-API node, not yet filed).** `add_decl` and
  `remove_last` stay `pub`, and eight tests install `Decl::Transparent`
  directly. In production, `add_decl` installs only opaque declarations.
