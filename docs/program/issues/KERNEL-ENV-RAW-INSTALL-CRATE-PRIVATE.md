---
id: KERNEL-ENV-RAW-INSTALL-CRATE-PRIVATE
title: "Code outside the kernel can still install a Decl::Transparent body or pop a declaration with no check, because GlobalEnv::add_decl and remove_last are pub. Route every external use through a checked kernel entry point and make both raw primitives crate-private"
status: active
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-ADMIT-BODIES-CHECKED-UPGRADE]
blocks: []
github: null
origin: "Carry from KERNEL-ADMIT-BODIES-CHECKED-UPGRADE (merged 84dc2f602): add_decl and remove_last stay pub, and eight tests install Decl::Transparent directly. Operator 2026-09-30 ('concur with recs'): a kernel node, after KERNEL-CONV-IOTA-DISCHARGE-DESCENT. Steward-filed per COORDINATION section 2."
---

# No raw declaration install outside the kernel

## Objective

Only the kernel can add a declaration to `GlobalEnv` or remove one. A body
reaches `Decl::Transparent` only through `check::admit_bodies`.

## Settled inputs (read at `895f0b5ce`)

- `GlobalEnv::add_decl` (`crates/ken-kernel/src/env.rs:397`) and
  `remove_last` (`:477`) are `pub`.
- **External production uses.**
  - `add_decl`: `ken-elaborator` `elab.rs` (3) and `compiler_driver.rs`
    (1).
  - `remove_last`: `data.rs:94` and `:294`, and `elab.rs:13860`.
- **External test uses.** `add_decl` appears in `k5_absurd_trusted_base.rs`
  and `l7_acceptance.rs`, plus three kernel integration tests. The
  admission node counted eight direct `Decl::Transparent` test installs.
- **Production installs only opaque declarations** through `add_decl` (the
  admission node's measurement). The hole is latent, like
  `discharge_hole`'s was.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`add_decl` and `remove_last` are `pub(crate)`. Each external use moves to
a checked kernel entry point that the Architect names. The eight
Transparent test installs go through `admit_bodies` or a test-only setup
that cannot reach production.

## Acceptance

- **AC-0 (census and design; no build).**
  - List every external use at the landed base, and what each one
    installs or removes.
  - Propose the checked entry point for each use.
  - The Architect rules before any edit.
- **AC-1.**
  - A compile check shows no crate outside `ken-kernel` can name either
    primitive.
  - An external attempt to install a `Decl::Transparent` body without a
    check has no path.
- **AC-2 (controls).**
  - The 57-package census shows no verdict change.
  - `trusted_base()` is unchanged on the targeted suites.

## Stop conditions

- Any change to what the kernel accepts: stop to the Architect.
- Any spec change: an operator question.
- A production use that has no checked equivalent: stop to the Architect
  with the site.
