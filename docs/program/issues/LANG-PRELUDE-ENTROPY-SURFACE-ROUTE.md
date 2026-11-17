---
id: LANG-PRELUDE-ENTROPY-SURFACE-ROUTE
title: "The prelude checks an entropy surface (EntropyIO, random_bytes, host_entropy, ambient_entropy) that no catalog package provides, so once Legacy resolution is deleted, Strict source cannot name it and three test consumers lose their route. Measure every public-shaped prelude action surface with no Strict route, then give each one a ruled route: a capability package home with its spec taxonomy row, or an internal route for its consumers"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-DELETE-LEGACY-RESOLUTION]
github: null
origin: "Architect evt_2kb4r05x2dc1y on LANG-PRELUDE-NONHOME-FAMILIES-HOME's HR-iii stop (language-implementer evt_44d017h395s4r): a Steward carry, owed before LANG-DELETE I4 removes the Legacy route. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# Entropy, and any other prelude action with no package

## Objective

Every public-shaped prelude term that is not a floor member, a strict
native builtin, a homed name or private has a ruled Strict route before
LANG-DELETE I4 deletes Legacy. Entropy is the known member. Each member
either gets a capability package home with its spec taxonomy row, or is
ruled internal, with its consumers moved to an internal route.

## Settled inputs (main `943d14a06`)

- **The known member.** `random_bytes` is a checked prelude proc
  (`crates/ken-elaborator/src/prelude.rs:1922`) of type
  `EntropyIO (Result IOError Bytes) visits [Entropy]`. `git grep -w`
  over `catalog/packages` for `random_bytes`, `EntropyIO`, `EntropyOp`,
  `host_entropy`, `ambient_entropy` and `Entropy` gives no hits. In
  `spec/`, `Entropy` appears only as an effect-row label
  (`36-effects.md:932`).
- **The consumers.** `crates/ken-elaborator/tests/lang_let_hostio_action.rs`
  (`clock_entropy_and_multilabel_inclusions_use_checked_helpers`, held
  out of LANG-PRELUDE-NONHOME-FAMILIES-HOME),
  `crates/ken-elaborator/tests/abi_s3_entropy_effect_row.rs` and
  `crates/ken-interp/tests/rt_c5_host_response_result_slots.rs:851`. A
  Strict client spelling `random_bytes` refuses `UnboundName`
  (`/workspaces/ken/tmp/nonhome-hr-entropy-strict.log`).
- **The precedent home.** `Capability.Time.WallClock` homes the clock
  action `wall_now` and, after the non-home WP, its alias `ClockIO`.
- **Why the census missed it.** The non-home census counted formers in
  public exports' checked types. No export touches entropy, so that
  census could not reach it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The class sweep, the Architect's per-member ruling, then either the home
rows, exports, spec taxonomy row and seed for each homed member, or the
consumer migration to an internal route for each internal member. No new
law, identity or kernel change, and no prelude identity moves.

## Deliverable

One candidate on the Language ring, with Spec enclave halves for any
home. If the sweep finds more than entropy, the Architect may rule a
split.

## Acceptance

- **AC-0 (D0, before code).** On the landed base, list every
  public-shaped prelude term name that is not floor, homed, strict
  builtin or private, and that no catalog package provides. For each,
  give its checked id, its type and its consumers in `crates/*/tests`,
  `conformance/` and `examples/`. The Architect rules home or internal
  per member, and names each home's owner package.
- **AC-1 (home).** For each homed member: a Strict client that imports it
  is accepted, keyed on the pre-source GlobalId, and the same client
  without the import refuses `UnboundName`. Its representation formers
  (for example `EntropyOp`) stay unbound.
- **AC-2 (internal).** For each internal member, every consumer named in
  AC-0 passes on its internal route, and a Strict client spelling the
  name still refuses `UnboundName`.
- **AC-3.** The held `lang_let_hostio_action` row migrates whole, with its
  promise unchanged across its three helpers.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A member's route needs a new identity, a kernel change or a trust
  change.
- A consumer's promise cannot be kept on either route.

Checks: 3 fired. The Architect's grep of the six entropy spellings over
`catalog/packages` gave 0 hits, and the three consumers are from the same
sweep. 6 fired: the population is all public-shaped prelude term names,
not only entropy, because the earlier census's membership rule could not
see a surface that no export names. AC-0 measures the class before any
route is chosen. 8 fired: AC-1's paired no-import client and AC-2's
Strict refusal tell a real route apart from an accidental Legacy
fallback.
