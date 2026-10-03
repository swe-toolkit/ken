---
id: KERNEL-ADMIT-BODIES-UPGRADABLE-ONLY
title: "The kernel proves Bottom with an empty trusted base, because admit_bodies upgrades any Opaque declaration: it will give the prelude Bottom the body Top, or re-upgrade a recursion-barrier fold that earlier declarations were checked against. Upgrade only a hole the kernel staged or recorded as an assumption"
status: ready
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-LEVEL-CLOSURE-CHECK]
blocks: []
github: null
origin: "Adversary evt_7ekcmvzdahkxs on 742f1b1a7 (pre-existing; same class as Research S1). Restores the merged KERNEL-ADMIT-BODIES-CHECKED-UPGRADE objective, a fail-closed narrowing inside it. Steward-filed per COORDINATION section 2."
---

# admit_bodies upgrades only a staged or recorded hole

## Objective

`admit_bodies` refuses every id that the kernel did not stage as a pending
placeholder or record as an assumption. Nothing can make a prelude constant
inhabited, or replace a body that other declarations were checked against.
This restores the objective of KERNEL-ADMIT-BODIES-CHECKED-UPGRADE: a body
may only retire a hole the kernel admitted.

## Settled inputs (Adversary `evt_7ekcmvzdahkxs`, measured on `742f1b1a7`)

- **The guard.** `check.rs:1430` checks only `Some(Decl::Opaque { .. })`.
- **Opaque producers** (the fan-in):
  - `declare_postulate`, which is recorded in `trusted_base()`;
  - `stage_placeholders`, which is pending;
  - `declare_prelude_const` (`env.rs:503`), which is not recorded;
  - the `with_recursion_barriers` folds (`env.rs:435`), which are not
    recorded.
- **Repros**, through the public API only:
  - P1: `admit_bodies(&[(bottom, Top)])` returns Ok. Then `tt : Bottom`
    checks, and `trusted_base()` is `[]` before and after.
  - P2: `Top := Bottom`, the same result.
  - P3: on a `with_recursion_barriers` clone, re-upgrade `rec` (which always
    returns 0) to `λn. 1`. A lemma checked earlier, `Eq Nat (rec 1) 0`, then
    types at Bottom, and `trusted_base` goes from `[rec]` to `[]`.
- **Reach.** The elaborator reaches it through `discharge_hole`
  (`ken-elaborator/src/lib.rs:559`) with a `pub` `Obligation.hole_id`. It is
  not reachable from `.ken` source.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

At AC-0, list every Opaque producer with its upgrade eligibility. Then
`admit_bodies` refuses an ineligible member with `IllFormedDecl`, before any
check or mutation. Only two kinds are eligible: a placeholder staged and still
pending, and a hole recorded as an assumption. Key eligibility on
kernel-owned state that the env tracks by `GlobalId`. Do not key it on
`Decl` shape or on spelling (check 10). A barrier fold is never eligible.

## Acceptance

- **AC-1.** P1, P2 and P3 are refused, and the env and `trusted_base()` are
  unchanged. A staged placeholder still upgrades. A postulate hole discharged
  through `discharge_hole` still leaves `trusted_base()`.
- **AC-2 (falsifier).** Restoring the bare Opaque guard reddens P1 and P3,
  while the two eligible controls stay green.
- **AC-3.** The kernel and elaborator suites that discharge holes stay green,
  and `trusted_base()` is unchanged on a fresh env.

## Stop conditions

- A production caller upgrades an id outside the two eligible kinds: stop to
  the Architect with the site.
- A spec change. Spec 18 §5 already excludes this.
